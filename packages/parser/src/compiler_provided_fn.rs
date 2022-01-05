use ast::data_type::DataType;
use indexmap::IndexMap;

use crate::symbol_table::{SymbolContext, SymbolMetaInsert};

pub fn insert_compiler_provided_fn(context: &mut SymbolContext) {
    /*
     * print(s : String);
     *
     * Prints s to stdout without newline and returns that string
     *
     * */

    let print_fn_arguments = {
        let mut i: IndexMap<String, DataType> = IndexMap::new();

        i.insert("s".to_string(), DataType::String);

        i
    };

    let print_data_type = DataType::FunctionType {
        return_type: Box::new(DataType::String),
        arguments: print_fn_arguments,
    };

    context
        .insert("print", SymbolMetaInsert::create(print_data_type, true))
        .unwrap();
}
