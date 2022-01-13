use std::collections::HashMap;

use inkwell::values::ptr_value::PointerValue;

#[derive(Debug)]
pub struct SymbolTable<'a> {
    pub global_variables: HashMap<String, PointerValue<'a>>,
    pub local_symbols: HashMap<String, PointerValue<'a>>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        return SymbolTable {
            global_variables: HashMap::new(),
            local_symbols: HashMap::new(),
        };
    }

    pub fn insert_local(&mut self, name: String, value: PointerValue<'a>) {
        self.local_symbols.insert(name, value);
    }

    pub fn insert_global(&mut self, name: String, value: PointerValue<'a>) {
        self.global_variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&PointerValue<'a>> {
        if self.local_symbols.contains_key(name) {
            return self.local_symbols.get(name);
        } else {
            return self.global_variables.get(name);
        }
    }
}
