use std::collections::HashMap;

use ast::Ast;
use lexer::convert_to_token;

use crate::{convert_to_ast_with_resolver, symbol_table::SymbolMetaInsert};

#[derive(Debug, Clone, PartialEq)]
pub struct ParserResolverData {
    pub symbol_table: HashMap<String, SymbolMetaInsert>,
    pub ast: Vec<Ast>,
}

impl ParserResolverData {
    pub fn new() -> Self {
        return ParserResolverData {
            symbol_table: HashMap::new(),
            ast: Vec::new(),
        };
    }
}

pub struct ParserResolver {
    main_data: Option<ParserResolverData>,
    map: HashMap<String, ParserResolverData>,
    dependencies: HashMap<String, String>,
    get_new_dependencies: Box<dyn Fn(&str) -> Result<String, ()>>,
}

impl ParserResolver {
    pub fn new() -> ParserResolver {
        return ParserResolver {
            main_data: None,
            map: HashMap::new(),
            dependencies: HashMap::new(),
            get_new_dependencies: Box::new(|_s: &str| return Err(())),
        };
    }

    pub fn from(
        dependencies: HashMap<String, String>,
        get_new_dependencies: Box<dyn Fn(&str) -> Result<String, ()>>,
    ) -> ParserResolver {
        return ParserResolver {
            main_data: None,
            map: HashMap::new(),
            dependencies,
            get_new_dependencies: get_new_dependencies,
        };
    }

    pub fn parse_data(&mut self, file_name: &str) {
        let source_code = self.get_dependent_source_code(file_name).unwrap();
        let tokens = convert_to_token(source_code);
        let (ast, symbols) = convert_to_ast_with_resolver(tokens, self);
        let resolver_data = ParserResolverData {
            ast,
            symbol_table: symbols,
        };
        self.map.insert(file_name.to_string(), resolver_data);
    }

    fn get_dependent_source_code(&mut self, file_name: &str) -> Option<&String> {
        if self.dependencies.contains_key(file_name) {
            return self.dependencies.get(file_name);
        } else {
            let new_dependencies = (self.get_new_dependencies)(file_name);
            if let Ok(s) = new_dependencies {
                self.dependencies.insert(file_name.to_string(), s);
                return self.dependencies.get(file_name);
            } else {
                return None;
            }
        }
    }

    pub fn get_data(&self, file_name: &str) -> &ParserResolverData {
        return self.map.get(file_name).unwrap();
    }

    pub fn contains_data(&self, file_name: &str) -> bool {
        return self.map.contains_key(file_name);
    }

    pub fn set_main(&mut self, data: ParserResolverData) {
        self.main_data = Some(data);
    }

    pub fn get_main(&self) -> &Option<ParserResolverData> {
        return &self.main_data;
    }

    pub fn get_dependencies(&self) -> Vec<String> {
        return self
            .dependencies
            .iter()
            .map(|(name, _)| {
                return name.to_string();
            })
            .collect();
    }
}
