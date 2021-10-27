use std::collections::HashMap;

use ast::{data_type::DataType, declaration::Declaration, Ast};
use indexmap::IndexMap;
use inkwell::{
    context::Context, module::Module, types::enums::BasicTypeEnum, values::ptr_value::PointerValue,
};

use crate::{build_expression::build_expression, llvm_utils::LLVMUtils};

use super::consume_single_ast;

pub(crate) fn consume_function_declaration<'a>(
    arguments: &IndexMap<String, DataType>,
    blocks: &Box<Vec<Ast>>,
    ident_name: &String,
    return_type: &DataType,
    context: &'a Context,
    module: &Module,
) {
    let mut number_of_arguments = 0;
    let llvm_return_type = return_type.to_basic_type(context);
    let param_types: Vec<BasicTypeEnum> = arguments
        .iter()
        .map(|(_, data_type)| {
            number_of_arguments += 1;
            return data_type.to_basic_type(context);
        })
        .collect();
    let fn_type = llvm_return_type.fn_type(&param_types, false);

    let mut function_value = module.add_function(ident_name, fn_type, None);

    /*
     * When we declare a function with arguments llvm assigns registers with name from 0 to ...
     * I could not find any way to change the name so now we have to make sure
     * when we call function_value.get_unique_reg_name() we do not return a name
     * which is already in use
     *
     * Thats why we have to increase the counter
     *
     *
     * */
    function_value.set_reg_counter(number_of_arguments);

    let entry_block = context.append_basic_block(&function_value, "main");
    let builder = context.create_builder();
    builder.position_at_end(&entry_block);

    let mut symbol_table: HashMap<String, PointerValue> = HashMap::new();

    /*
     * We have to store parameters in stack so that it can be
     * manipulated just like any other variables 
     * 
     * */
    for (i, (name, data_type)) in arguments.iter().enumerate() {
        let llvm_type = data_type.to_basic_type(context);
        let arg_pointer = builder.build_alloca(llvm_type, name);

        let param_value = function_value.get_nth_param(i as u32).unwrap();
        builder.build_store(arg_pointer, param_value);

        symbol_table.insert(name.to_string(), arg_pointer);
    }

    for cur_ast in blocks.as_ref() {
        if let Ast::Declaration(dec) = cur_ast {
            match dec {
                Declaration::ReturnStatement { return_exp } => {
                    let value = build_expression(
                        return_exp,
                        context,
                        &builder,
                        &mut function_value,
                        &mut symbol_table,
                        None,
                    );
                    builder.build_return(Some(&value));
                }

                _ => consume_single_ast(
                    cur_ast,
                    context,
                    &builder,
                    &mut function_value,
                    &mut symbol_table,
                ),
            }
        }
    }
}
