use std::collections::HashMap;

use ast::data_type::DataType;

pub struct SymbolMeta {
    pub data_type: DataType,
    pub is_const: bool,
    pub is_override_available: bool,
    pub suffix: String,
}

pub struct SymbolMetaInsert {
    data_type: DataType,
    is_const: bool,
}

impl SymbolMetaInsert {
    pub fn create(data_type: DataType, is_const: bool) -> Self {
        return SymbolMetaInsert {
            data_type,
            is_const,
        };
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSymbol {
    return_type: DataType,
}

impl FunctionSymbol {
    pub fn new(return_type: DataType) -> Self {
        return Self { return_type };
    }
}

pub struct SymbolContext<'a> {
    symbols: HashMap<String, SymbolMetaInsert>,
    parent: Option<Box<&'a SymbolContext<'a>>>,
    function_symbol: Option<FunctionSymbol>,

    pub suffix: String,
    pub counter: usize,

    temp_counter: usize,
}

impl<'a> SymbolContext<'a> {
    pub fn create_global_context() -> Self {
        return SymbolContext {
            symbols: HashMap::new(),
            parent: None,
            function_symbol: None,

            suffix: String::from("_"),
            counter: 0,

            temp_counter: 0,
        };
    }

    pub fn create_function_context(function_symbol: FunctionSymbol) -> Self {
        return SymbolContext {
            symbols: HashMap::new(),
            parent: None,
            function_symbol: Some(function_symbol),

            suffix: String::from("_"),
            counter: 0,

            temp_counter: 0,
        };
    }

    pub fn get(&self, name: &str) -> Option<SymbolMeta> {
        let context_available = self.get_context_for_name(name);

        match context_available {
            None => return None,
            Some(context) => {
                let is_override = context.suffix != self.suffix;
                let meta_insert = context.symbols.get(&name.to_string()).unwrap();
                let meta = SymbolMeta {
                    data_type: meta_insert.data_type.clone(),
                    is_const: meta_insert.is_const,
                    is_override_available: is_override,
                    suffix: self.get_suffix(name),
                };

                return Some(meta);
            }
        }
    }

    pub fn insert(&mut self, name: &str, sym_meta: SymbolMetaInsert) -> Result<(), String> {
        let key = name.to_string();

        if self.symbols.contains_key(&key) {
            return Err(format!("There is already a variable with key {}", key));
        } else {
            self.symbols.insert(key, sym_meta);
            return Ok(());
        }
    }

    pub fn create_child_context(&'a self, suffix: String) -> SymbolContext<'a> {
        let new_context = SymbolContext {
            symbols: HashMap::new(),
            parent: Some(Box::new(self)),
            function_symbol: self.function_symbol.clone(),
            suffix,
            counter: 0,
            temp_counter: 0,
        };
        return new_context;
    }

    pub fn get_suffix(&self, name: &str) -> String {
        let context_available = self.get_context_for_name(name).unwrap();
        return context_available.suffix.clone();
    }

    pub fn get_return_type(&self) -> Option<&DataType> {
        match &self.function_symbol {
            None => None,
            Some(sym) => return Some(&sym.return_type),
        }
    }

    fn get_context_for_name(&'a self, name: &str) -> Option<&'a SymbolContext<'a>> {
        let mut cur_context = &Some(Box::new(self));

        loop {
            match cur_context {
                None => return None,

                Some(context) => {
                    let is_present = context.symbols.contains_key(&name.to_string());

                    if is_present {
                        return Some(context.as_ref());
                    } else {
                        cur_context = &context.parent;
                    }
                }
            }
        }
    }

    pub fn get_temp_name(&mut self) -> String {
        let temp_counter = self.temp_counter;
        let temp_name = format!("|{}temp{}", self.suffix, temp_counter);
        self.temp_counter += 1;

        return temp_name;
    }
}
