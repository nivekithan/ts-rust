use ast::data_type::DataType;
use indexmap::IndexMap;
use inkwell::{context::Context, module::Module};

use crate::{symbol_table::SymbolTable, utils::convert_function_data_type_to_llvm_function_type};

pub(crate) fn consume_import_declaration<'a>(
    idents: &IndexMap<String, DataType>,
    context: &'a Context,
    symbol_table: &mut SymbolTable<'a>,
    module: &'a Module,
) {
    for (name, data_type) in idents {
        if let DataType::FunctionType {
            arguments: _,
            return_type: _,
        } = data_type
        {
            let fn_type = convert_function_data_type_to_llvm_function_type(data_type, context);
            let fn_value = module.add_function(name, fn_type, None);
            symbol_table.insert_global(name.to_string(), fn_value.to_pointer_value());
        } else {
            panic!("Currently it is only supported to import functions");
        }
    }
}
