use std::collections::HashMap;

use ast::{
    data_type::DataType,
    declaration::{Declaration, VariableAssignmentOperator},
    Ast,
};
use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    types::traits::BasicTypeTrait,
    values::{enums::BasicValueEnum, fn_value::FunctionValue, ptr_value::PointerValue},
};

use crate::{
    build_expression::build_expression,
    enums::{NextElsIfBlock, TypeOfIfBlock},
};

pub(crate) fn consume_ast<'a>(
    asts: &Vec<Ast>,
    context: &'a Context,
    builder: &'a Builder,
    function_value: &mut FunctionValue,
    symbol_table: &mut HashMap<String, PointerValue<'a>>,
) {
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
                            let pointer =
                                builder.build_alloca(context.f64_type(), ident_name.as_str());
                            let value_of_exp = build_expression(
                                exp,
                                context,
                                builder,
                                function_value,
                                symbol_table,
                                None,
                            );
                            builder.build_store(pointer.clone(), value_of_exp);

                            pointer
                        }

                        DataType::Boolean => {
                            let pointer =
                                builder.build_alloca(context.i1_type(), ident_name.as_str());
                            let value_of_exp = build_expression(
                                exp,
                                context,
                                builder,
                                function_value,
                                symbol_table,
                                None,
                            );
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
                    let value =
                        build_expression(exp, context, builder, function_value, symbol_table, None);

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
                }

                Declaration::NewIfBlockDeclaration {
                    else_block,
                    else_if_block,
                    if_block,
                } => {
                    let if_block_condition = build_expression(
                        &if_block.condition,
                        context,
                        builder,
                        function_value,
                        symbol_table,
                        None,
                    );

                    if let BasicValueEnum::IntValue(cond_value) = if_block_condition {
                        let width = cond_value.get_type().get_bit_width();

                        if width != 1 {
                            panic!(
                                "Expected the width of condition_value to be 1 but got {}",
                                width
                            );
                        }

                        let if_block_name = function_value.get_unique_block_name();
                        let if_block_bb =
                            context.append_basic_block(function_value, if_block_name.as_str());

                        let else_if_block_bbs: Vec<(BasicBlock, BasicBlock)> = else_if_block
                            .iter()
                            .map(|_| {
                                let condition_checker_block_name =
                                    function_value.get_unique_block_name();
                                let condition_checker_block = context.append_basic_block(
                                    function_value,
                                    condition_checker_block_name.as_str(),
                                );
                                let block_name = function_value.get_unique_block_name();
                                let block =
                                    context.append_basic_block(function_value, block_name.as_str());
                                return (condition_checker_block, block);
                            })
                            .collect();

                        let else_block_maybe: Option<BasicBlock> = match else_block {
                            None => None,
                            Some(_) => {
                                let else_block_name = function_value.get_unique_block_name();
                                let block = context
                                    .append_basic_block(function_value, else_block_name.as_str());
                                Some(block)
                            }
                        };

                        let exit_block_name = function_value.get_unique_block_name();
                        let exit_block_bb =
                            context.append_basic_block(function_value, exit_block_name.as_str());

                        let type_of_if_block = {
                            if else_if_block_bbs.len() == 0 && matches!(else_block_maybe, None) {
                                TypeOfIfBlock::IfBlock
                            } else if else_if_block.len() == 0 && !matches!(else_block_maybe, None)
                            {
                                TypeOfIfBlock::IfAndElse
                            } else if else_if_block.len() != 0 && matches!(else_block_maybe, None) {
                                TypeOfIfBlock::IfAndElseIf
                            } else {
                                TypeOfIfBlock::IfElseIfAndElse
                            }
                        };

                        match type_of_if_block {
                            TypeOfIfBlock::IfBlock => {
                                builder.build_conditional_branch(
                                    cond_value,
                                    &if_block_bb,
                                    &exit_block_bb,
                                );
                                builder.position_at_end(&if_block_bb);

                                consume_ast(
                                    &if_block.block,
                                    context,
                                    builder,
                                    function_value,
                                    symbol_table,
                                );

                                builder.build_unconditional_branch(&exit_block_bb);
                                builder.position_at_end(&exit_block_bb);
                            }

                            TypeOfIfBlock::IfAndElse => {
                                if let Some(else_block_bb) = else_block_maybe {
                                    if let Some(else_block_asts) = else_block {
                                        builder.build_conditional_branch(
                                            cond_value,
                                            &if_block_bb,
                                            &else_block_bb,
                                        );
                                        builder.position_at_end(&if_block_bb);

                                        consume_ast(
                                            &if_block.block,
                                            context,
                                            builder,
                                            function_value,
                                            symbol_table,
                                        );

                                        builder.build_unconditional_branch(&exit_block_bb);

                                        builder.position_at_end(&else_block_bb);

                                        consume_ast(
                                            else_block_asts,
                                            context,
                                            builder,
                                            function_value,
                                            symbol_table,
                                        );

                                        builder.build_unconditional_branch(&exit_block_bb);
                                        builder.position_at_end(&exit_block_bb);
                                    }
                                } else {
                                    unreachable!()
                                }
                            }

                            TypeOfIfBlock::IfAndElseIf | TypeOfIfBlock::IfElseIfAndElse => {
                                let get_next_block = |i: usize| {
                                    if else_if_block_bbs.len() <= i + 1 {
                                        if !matches!(&else_block_maybe, None) {
                                            if let Some(else_block_bb) = &else_block_maybe {
                                                return NextElsIfBlock::Else(else_block_bb);
                                            } else {
                                                unreachable!()
                                            }
                                        }
                                        return NextElsIfBlock::Exit(&exit_block_bb);
                                    } else {
                                        return NextElsIfBlock::ElseIfBlock(
                                            &else_if_block_bbs[i + 1],
                                        );
                                    }
                                };

                                let (condition_checker_block_bb, _) = &else_if_block_bbs[0];

                                builder.build_conditional_branch(
                                    cond_value,
                                    &if_block_bb,
                                    condition_checker_block_bb,
                                );
                                builder.position_at_end(&if_block_bb);

                                consume_ast(
                                    &if_block.block,
                                    context,
                                    builder,
                                    function_value,
                                    symbol_table,
                                );
                                builder.build_unconditional_branch(&exit_block_bb);
                                builder.position_at_end(condition_checker_block_bb);

                                for (i, (_, else_if_block_bb)) in
                                    else_if_block_bbs.iter().enumerate()
                                {
                                    let conditional_value = build_expression(
                                        &else_if_block[i].condition,
                                        context,
                                        builder,
                                        function_value,
                                        symbol_table,
                                        None,
                                    );

                                    if let BasicValueEnum::IntValue(cond_value) = conditional_value
                                    {
                                        let next_branch = get_next_block(i);

                                        match next_branch {
                                            NextElsIfBlock::Exit(exit_block_bb) => {
                                                builder.build_conditional_branch(
                                                    cond_value,
                                                    else_if_block_bb,
                                                    exit_block_bb,
                                                );
                                                builder.position_at_end(else_if_block_bb);

                                                consume_ast(
                                                    &else_if_block[i].block,
                                                    context,
                                                    builder,
                                                    function_value,
                                                    symbol_table,
                                                );
                                                builder.build_unconditional_branch(&exit_block_bb);
                                                builder.position_at_end(&exit_block_bb);
                                                break;
                                            }

                                            NextElsIfBlock::ElseIfBlock((
                                                next_condition_checker_block_bb,
                                                _,
                                            )) => {
                                                builder.build_conditional_branch(
                                                    cond_value,
                                                    else_if_block_bb,
                                                    next_condition_checker_block_bb,
                                                );
                                                builder.position_at_end(else_if_block_bb);

                                                consume_ast(
                                                    &else_if_block[i].block,
                                                    context,
                                                    builder,
                                                    function_value,
                                                    symbol_table,
                                                );
                                                builder.build_unconditional_branch(&exit_block_bb);
                                                builder.position_at_end(
                                                    next_condition_checker_block_bb,
                                                );
                                            }

                                            NextElsIfBlock::Else(else_block_bb) => {
                                                builder.build_conditional_branch(
                                                    cond_value,
                                                    else_if_block_bb,
                                                    else_block_bb,
                                                );
                                                builder.position_at_end(else_if_block_bb);
                                                consume_ast(
                                                    &else_if_block[i].block,
                                                    context,
                                                    builder,
                                                    function_value,
                                                    symbol_table,
                                                );
                                                builder.build_unconditional_branch(&exit_block_bb);
                                                builder.position_at_end(else_block_bb);
                                            }
                                        }
                                    }
                                }

                                if let Some(_) = else_block_maybe {
                                    if let Some(else_block_ast) = else_block {
                                        consume_ast(
                                            else_block_ast,
                                            context,
                                            builder,
                                            function_value,
                                            symbol_table,
                                        );
                                        builder.build_unconditional_branch(&exit_block_bb);
                                        builder.position_at_end(&exit_block_bb);
                                    } else {
                                        unreachable!()
                                    }
                                }
                            }
                        }
                    }
                }

                _ => todo!(),
            },

            _ => todo!(),
        }
    }
}
