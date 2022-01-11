use std::collections::HashMap;

use lexer::convert_to_token;

use crate::{convert_to_ast_with_resolver, symbol_table::SymbolMetaInsert};

#[derive(Debug, Clone, PartialEq)]
pub struct Resolver {
    container_for_strings: HashMap<String, String>,
    container_for_context: HashMap<String, HashMap<String, SymbolMetaInsert>>,
}

impl Resolver {
    pub fn new() -> Resolver {
        return Resolver {
            container_for_strings: HashMap::new(),
            container_for_context: HashMap::new(),
        };
    }

    pub fn from<'b>(container_for_strings: HashMap<String, String>) -> Resolver {
        let mut resolver = Resolver {
            container_for_strings,
            container_for_context: HashMap::new(),
        };
        resolver.create_tokens_for_all_source_code();
        return resolver;
    }

    pub fn create_tokens_for_all_source_code(&mut self) {
        for (file_name, source_code) in &self.container_for_strings {
            let tokens = convert_to_token(source_code);
            let (_, context) = convert_to_ast_with_resolver(tokens, self.clone());
            self.container_for_context
                .insert(file_name.to_string(), context);
        }
    }

    pub fn get_symbols(&self, file_name: &str) -> &HashMap<String, SymbolMetaInsert> {
        if self.container_for_context.contains_key(file_name) {
            return self.container_for_context.get(file_name).unwrap();
        } else {
            panic!("Call create_tokens_for_source_code before calling get_token_for")
        }
    }
}
