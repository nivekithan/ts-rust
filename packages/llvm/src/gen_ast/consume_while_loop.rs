use std::collections::HashMap;

use ast::{expression::Expression, Ast};
use inkwell::{
    builder::Builder,
    context::Context,
    values::{enums::BasicValueEnum, fn_value::FunctionValue, ptr_value::PointerValue},
};

use crate::build_expression::build_expression;

use super::consume_ast;

pub(crate) fn consume_while_loop<'a>(
    block: &Box<Vec<Ast>>,
    condition: &Expression,
    context: &'a Context,
    builder: &'a Builder,
    function_value: &mut FunctionValue,
    symbol_table: &mut HashMap<String, PointerValue<'a>>,
) {
    let condition_checker_block_name = function_value.get_unique_block_name();
    let condition_checker_block_bb =
        context.append_basic_block(function_value, condition_checker_block_name.as_str());

    let while_block_name = function_value.get_unique_block_name();
    let while_block_bb = context.append_basic_block(function_value, while_block_name.as_str());

    let exit_block_name = function_value.get_unique_block_name();
    let exit_block_bb = context.append_basic_block(function_value, exit_block_name.as_str());

    builder.build_unconditional_branch(&condition_checker_block_bb);
    builder.position_at_end(&condition_checker_block_bb);

    let condition_value = build_expression(
        condition,
        context,
        builder,
        function_value,
        symbol_table,
        None,
    );

    if let BasicValueEnum::IntValue(cond_value) = condition_value {
        builder.build_conditional_branch(cond_value, &while_block_bb, &exit_block_bb);
        builder.position_at_end(&while_block_bb);
        consume_ast(block, context, builder, function_value, symbol_table);
        builder.build_unconditional_branch(&condition_checker_block_bb);
        builder.position_at_end(&exit_block_bb);
    } else {
        todo!()
    }
}