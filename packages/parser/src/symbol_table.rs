use std::collections::HashMap;

use ast::data_type::DataType;

pub struct SymbolMeta {
    pub data_type: DataType,
    pub is_const: bool,
    pub is_override_available: bool,
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

pub struct SymbolContext<'a> {
    symbols: HashMap<String, SymbolMetaInsert>,
    parent: Option<Box<&'a SymbolContext<'a>>>,
}

impl<'a> SymbolContext<'a> {
    pub fn new_global() -> Self {
        return SymbolContext {
            symbols: HashMap::new(),
            parent: None,
        };
    }

    pub fn get(&self, name: &str) -> Option<SymbolMeta> {
        let mut cur_context: Option<&SymbolContext> = Some(self);
        let mut is_override = false;

        while !matches!(cur_context, None) {
            if let Some(context) = cur_context {
                let sym_meta = context.symbols.get(&name.to_string());

                match sym_meta {
                    Some(meta) => {
                        return Some(SymbolMeta {
                            data_type: meta.data_type.clone(),
                            is_const: meta.is_const,
                            is_override_available: is_override,
                        })
                    }

                    None => {
                        if let Some(context) = &context.parent {
                            is_override = true;
                            cur_context = Some(context.as_ref());
                        } else {
                            cur_context = None;
                        }
                    }
                }
            } else {
                unreachable!();
            }
        }

        return None;
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

    pub fn create_child_context(&'a self) -> Self {
        let new_context = SymbolContext {
            symbols: HashMap::new(),
            parent: Some(Box::new(self)),
        };
        return new_context;
    }
}
