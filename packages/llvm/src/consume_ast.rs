
use std::collections::HashMap;

use ast::{Ast, data_type::DataType, declaration::{Declaration, VariableAssignmentOperator}};
use inkwell::{builder::Builder, context::Context, types::traits::BasicTypeTrait, values::{enums::BasicValueEnum, fn_value::FunctionValue, ptr_value::PointerValue}};

use crate::build_expression::build_expression;



pub(crate) fn consume_ast<'a>(asts : &Vec<Ast>, context : &'a Context, builder : &'a Builder, function_value : &mut FunctionValue,  symbol_table : & mut HashMap<String, PointerValue<'a>>) {
    

    for cur_ast in asts.iter() {
        match cur_ast {
            Ast::Declaration(dec) => match dec {
                Declaration::VariableDeclaration {
                    ident_name,
                    exp,
                    kind: _,
                } => {
                    let data_type = exp.get_data_type();

                    let pointer = match data_type {
                        DataType::Float => {
                            let pointer = builder
                                .build_alloca(context.f64_type(), ident_name.as_str());
                            let value_of_exp =
                                build_expression(exp, context, builder, function_value, symbol_table, None);
                            builder.build_store(pointer.clone(), value_of_exp);

                            pointer
                        }

                        DataType::Boolean => {
                            let pointer = builder
                                .build_alloca(context.i1_type(), ident_name.as_str());
                            let value_of_exp =
                                build_expression(exp, context, builder, function_value, symbol_table, None);
                            builder.build_store(pointer.clone(), value_of_exp);

                            pointer
                        }

                        _ => todo!(),
                    };

                    symbol_table.insert(ident_name.to_owned(), pointer);
                }

                // Does not do typechecking
                Declaration::VariableAssignment {
                    ident_name,
                    exp,
                    operator,
                } => {
                    let var_ptr = symbol_table.get(ident_name).unwrap().clone();
                    let value = build_expression(exp, context, builder, function_value, symbol_table, None);

                    match operator {
                        VariableAssignmentOperator::Assign => {
                            builder.build_store(var_ptr, value);
                        }

                        VariableAssignmentOperator::PlusAssign => {
                            let load_value = builder.build_load(
                                var_ptr,
                                context.f64_type().as_basic_type_enum(),
                                function_value.get_unique_reg_name().as_str(),
                            );

                            if let BasicValueEnum::FloatValue(lhs) = load_value {
                                if let BasicValueEnum::FloatValue(rhs) = value {
                                    let result_value = builder.build_float_add(
                                        lhs,
                                        rhs,
                                        function_value.get_unique_reg_name().as_str(),
                                    );
                                    builder.build_store(var_ptr, result_value);
                                }
                            }
                        }

                        VariableAssignmentOperator::MinusAssign => {
                            let load_value = builder.build_load(
                                var_ptr,
                                context.f64_type().as_basic_type_enum(),
                                function_value.get_unique_reg_name().as_str(),
                            );

                            if let BasicValueEnum::FloatValue(lhs) = load_value {
                                if let BasicValueEnum::FloatValue(rhs) = value {
                                    let result_value = builder.build_float_sub(
                                        lhs,
                                        rhs,
                                        function_value.get_unique_reg_name().as_str(),
                                    );
                                    builder.build_store(var_ptr, result_value);
                                }
                            }
                        }

                        VariableAssignmentOperator::StarAssign => {
                            let load_value = builder.build_load(
                                var_ptr,
                                context.f64_type().as_basic_type_enum(),
                                function_value.get_unique_reg_name().as_str(),
                            );

                            if let BasicValueEnum::FloatValue(lhs) = load_value {
                                if let BasicValueEnum::FloatValue(rhs) = value {
                                    let result_value = builder.build_float_mul(
                                        lhs,
                                        rhs,
                                        function_value.get_unique_reg_name().as_str(),
                                    );
                                    builder.build_store(var_ptr, result_value);
                                }
                            }
                        }

                        VariableAssignmentOperator::SlashAssign => {
                            let load_value = builder.build_load(
                                var_ptr,
                                context.f64_type().as_basic_type_enum(),
                                function_value.get_unique_reg_name().as_str(),
                            );

                            if let BasicValueEnum::FloatValue(lhs) = load_value {
                                if let BasicValueEnum::FloatValue(rhs) = value {
                                    let result_value = builder.build_float_div(
                                        lhs,
                                        rhs,
                                        function_value.get_unique_reg_name().as_str(),
                                    );
                                    builder.build_store(var_ptr, result_value);
                                }
                            }
                        } // _ => todo!(),
                    };
                },

                Declaration::IfBlockDeclaration{block, condition} => {
                    let condition_value = build_expression(condition, context, builder, function_value, symbol_table, None);

                    if let BasicValueEnum::IntValue(cond_value) = condition_value {
                        let width = cond_value.get_type().get_bit_width();

                        if width != 1 {
                            panic!("Expected the width of condition_value to be 1 but got {}", width);
                        } 

                        let if_block_name = function_value.get_unique_block_name();
                        let if_block = context.append_basic_block(function_value, if_block_name.as_str());
                        
                        
                        let exit_block_name = function_value.get_unique_block_name();
                        let exit_block = context.append_basic_block(function_value, exit_block_name.as_str());

                        builder.build_conditional_branch(cond_value, &if_block, &exit_block);

                        builder.position_at_end(if_block);  
                        consume_ast(block.as_ref(), context, builder, function_value, symbol_table);

                        
                        builder.build_unconditional_branch(&exit_block);
                        
                        builder.position_at_end(exit_block);

                    } else {
                        panic!("Expected the value of condition to be of Int but got {:?}", condition_value);
                    }
                },
                // _ => todo!(),
            },

            _ => todo!(),
        }
    }

}