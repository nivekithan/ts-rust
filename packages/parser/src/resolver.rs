use std::collections::HashMap;

use ast::Ast;
use lexer::convert_to_token;

use crate::{convert_to_ast_with_resolver, symbol_table::SymbolMetaInsert};

#[derive(Debug, Clone, PartialEq)]
pub struct ResolverData {
    pub symbol_table: HashMap<String, SymbolMetaInsert>,
    pub ast: Vec<Ast>,
}

impl ResolverData {
    pub fn new() -> Self {
        return ResolverData {
            symbol_table: HashMap::new(),
            ast: Vec::new(),
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Resolver {
    main_data: Option<ResolverData>,

    map: HashMap<String, ResolverData>,
    dependencies: HashMap<String, String>,
}

impl Resolver {
    pub fn new() -> Resolver {
        return Resolver {
            main_data: None,
            map: HashMap::new(),
            dependencies: HashMap::new(),
        };
    }

    pub fn from(dependencies: HashMap<String, String>) -> Resolver {
        return Resolver {
            main_data: None,
            map: HashMap::new(),
            dependencies,
        };
    }

    pub fn parse_data(&mut self, file_name: &str) {
        let source_code = self.dependencies.get(file_name).unwrap();
        let tokens = convert_to_token(source_code);
        let (ast, symbols) = convert_to_ast_with_resolver(tokens, self);
        let resolver_data = ResolverData {
            ast,
            symbol_table: symbols,
        };
        self.map.insert(file_name.to_string(), resolver_data);
    }

    pub fn get_data(&self, file_name: &str) -> &ResolverData {
        return self.map.get(file_name).unwrap();
    }

    pub fn contains_data(&self, file_name: &str) -> bool {
        return self.map.contains_key(file_name);
    }

    pub fn set_main(&mut self, data: ResolverData) {
        self.main_data = Some(data);
    }

    pub fn get_main(&self) -> &Option<ResolverData> {
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
