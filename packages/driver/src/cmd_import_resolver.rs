use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use ast::Ast;
use inkwell::{context::Context, module::Module};
use lexer::convert_to_token;
use llvm::{compile_to_llvm_module, compiler_provided_fn::get_compiler_provided_module};
use parser::{consume_token, symbol_table::SymbolMetaInsert, traits::ImportResolver};
use path_absolutize::Absolutize;

use crate::{file_unique_id::FileUniqueId, utils::convert_to_absolute_path};

pub struct CommandLineResolver {
    symbol_db: HashMap<String, HashMap<String, SymbolMetaInsert>>,
    ast_db: HashMap<String, Vec<Ast>>,
    id_db: FileUniqueId,
}

impl<'a> CommandLineResolver {
    pub fn new() -> Self {
        return CommandLineResolver {
            symbol_db: HashMap::new(),
            ast_db: HashMap::new(),
            id_db: FileUniqueId::new(),
        };
    }

    pub fn get_main_file_path(&self) -> PathBuf {
        let name = std::env::args().nth(1).unwrap();
        return self.get_absolute_file_path(&name);
    }

    pub fn get_absolute_file_path(&self, name: &str) -> PathBuf {
        let mut cwd = std::env::current_dir().unwrap();
        cwd.push(Path::new(name));
        return Path::new(cwd.absolutize().unwrap().to_str().unwrap()).to_path_buf();
    }

    pub fn get_assembly_file_path(&self) -> PathBuf {
        let file_name = "./output.s";
        return self.get_absolute_file_path(file_name);
    }

    pub fn get_file_content(&self, file_path: &PathBuf) -> String {
        let content = fs::read_to_string(file_path).unwrap();
        return content;
    }

    pub fn compile_assembly_to_exec(&self) {
        let assembly_file_path = self.get_assembly_file_path();
        Command::new("gcc")
            .args([assembly_file_path.to_str().unwrap(), "-o", "output"])
            .status()
            .unwrap();
    }

    pub fn compile(&mut self, context: &'a Context) -> Module<'a> {
        let main_file_path = self.get_main_file_path();
        let main_file_name = main_file_path.to_str().unwrap().to_string();
        let main_file_content = self.get_file_content(&main_file_path);
        let main_tokens = convert_to_token(&main_file_content);

        self.id_db.insert_main(&main_file_name);

        let (main_ast, _) = consume_token(main_tokens, self, Some(&main_file_name));

        let main_llvm_module = compile_to_llvm_module(main_ast, &context, "main", true);

        for (file_name, ast) in &self.ast_db {
            let module = compile_to_llvm_module(ast.clone(), &context, file_name, false);
            main_llvm_module.link_module(module).unwrap();
        }

        let compiler_internal_module = get_compiler_provided_module(context);
        main_llvm_module
            .link_module(compiler_internal_module)
            .unwrap();

        return main_llvm_module;
    }

    fn resolve_imported_file_name(&self, relative_file_name: &str, cur_file_name: &str) -> String {
        let absolute_file_name = convert_to_absolute_path(
            relative_file_name,
            PathBuf::from(cur_file_name)
                .parent()
                .unwrap()
                .to_str()
                .unwrap(),
        );
        return absolute_file_name;
    }
}

impl ImportResolver for CommandLineResolver {
    fn contains(&self, relative_file_name: &str, cur_file_name: &str) -> bool {
        if cur_file_name == "compilerInternal" {
            return true;
        }

        let absolute_file_name = self.resolve_imported_file_name(relative_file_name, cur_file_name);
        return self.symbol_db.contains_key(&absolute_file_name);
    }

    fn get(
        &self,
        relative_file_name: &str,
        cur_file_name: &str,
    ) -> Option<&HashMap<String, SymbolMetaInsert>> {
        let absolute_path = self.resolve_imported_file_name(relative_file_name, cur_file_name);
        return self.symbol_db.get(&absolute_path);
    }

    fn resolve(&mut self, relative_file_name: &str, cur_file_name: &str) -> Result<(), String> {
        let absolute_file_name = self.resolve_imported_file_name(relative_file_name, cur_file_name);

        if !self.contains(relative_file_name, cur_file_name) {
            let file_content = self.get_file_content(&PathBuf::from(absolute_file_name.clone()));
            self.id_db.insert(&absolute_file_name);
            let (ast, table) = consume_token(
                convert_to_token(&file_content),
                self,
                Some(absolute_file_name.as_str()),
            );
            self.symbol_db.insert(absolute_file_name.clone(), table);
            self.ast_db.insert(absolute_file_name, ast);
            return Ok(());
        } else {
            return Err(format!(
                "There is already a file with path {}",
                absolute_file_name
            ));
        }
    }

    fn get_id_for_file_name(&self, absolute_file_name: &str) -> usize {
        return *self.id_db.get(absolute_file_name).unwrap();
    }

    fn get_id(&self, relative_file_name: &str, cur_file_name: &str) -> usize {
        let absolute_file_name = self.resolve_imported_file_name(relative_file_name, cur_file_name);
        return self.get_id_for_file_name(&absolute_file_name);
    }
}
