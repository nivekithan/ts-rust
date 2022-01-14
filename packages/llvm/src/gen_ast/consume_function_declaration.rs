use ast::{data_type::DataType, declaration::Declaration, Ast};
use indexmap::IndexMap;
use inkwell::{
    context::Context, module::Module, types::enums::BasicTypeEnum, values::enums::BasicValueEnum,
};

use crate::{build_expression::build_expression, llvm_utils::LLVMUtils, symbol_table::SymbolTable};

use super::consume_single_ast;

pub(crate) fn consume_function_declaration<'a>(
    arguments: &IndexMap<String, DataType>,
    blocks: &Box<Vec<Ast>>,
    ident_name: &String,
    return_type: &DataType,
    context: &'a Context,
    module: &'a Module,
    symbol_table: &mut SymbolTable<'a>,
) {
    let mut number_of_arguments = 0;
    let llvm_return_type = return_type.force_to_basic_type(context);
    let param_types: Vec<BasicTypeEnum> = arguments
        .iter()
        .map(|(_, data_type)| {
            number_of_arguments += 1;
            return data_type.force_to_basic_type(context);
        })
        .collect();
    let fn_type = llvm_return_type.fn_type(&param_types, false);

    let mut function_value = module.add_function(ident_name, fn_type, None);

    symbol_table.insert_global(ident_name.to_string(), function_value.to_pointer_value());

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

    let mut new_symbol_table = SymbolTable::new();
    new_symbol_table.global_variables = symbol_table.global_variables.clone();

    /*
     * We have to store parameters in stack so that it can be
     * manipulated just like any other variables
     *
     * */
    for (i, (name, data_type)) in arguments.iter().enumerate() {
        let llvm_type = data_type.force_to_basic_type(context);

        if let BasicTypeEnum::PointerType(_) = llvm_type {
            /*
             * Since typescript does not have pointers, pointers in parameters corresponds to
             *    => String
             *    => Object
             *    => Array
             *    => Function
             *
             * types in typescript
             *
             *
             *  There is no reason for us to allocate pointers in stack
             * and store the pointer in `symbol_table` since they are pointer themselves
             * so we can directly store pointers in `symbol_table`
             *
             *
             *
             * */

            let param_value = function_value.get_nth_param(i as u32).unwrap();
            if let BasicValueEnum::PointerValue(param_value) = param_value {
                new_symbol_table.insert_local(name.to_string(), param_value);
            } else {
                todo!();
            }
        } else {
            let arg_pointer = builder.build_alloca(llvm_type, name);

            let param_value = function_value.get_nth_param(i as u32).unwrap();
            builder.build_store(arg_pointer, param_value);

            new_symbol_table.insert_local(name.to_string(), arg_pointer);
        }
    }

    for cur_ast in blocks.as_ref() {
        if let Ast::Declaration(dec) = cur_ast {
            match dec {
                Declaration::ReturnStatement { return_exp } => {
                    // If the return_exp is Option::None then we have to return void

                    if let Some(return_exp) = return_exp {
                        let value = build_expression(
                            return_exp,
                            context,
                            &builder,
                            &mut function_value,
                            &mut new_symbol_table,
                            module,
                            None,
                        );
                        match value {
                            Some(value) => builder.build_return(Some(&value)),
                            None => builder.build_return(None),
                        };
                    } else {
                        builder.build_return(None);
                    }
                }

                _ => consume_single_ast(
                    cur_ast,
                    context,
                    &builder,
                    &mut function_value,
                    &mut new_symbol_table,
                    module,
                ),
            }
        }
    }
}
