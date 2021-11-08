use std::collections::HashMap;

use ast::{expression::Expression, Ast};
use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    values::{enums::BasicValueEnum, fn_value::FunctionValue, ptr_value::PointerValue},
};

use crate::build_expression::build_expression;

use super::consume_ast_in_loop;

pub(crate) fn consume_do_while_loop<'a>(
    block: &Box<Vec<Ast>>,
    condition: &Expression,
    context: &'a Context,
    builder: &'a Builder,
    function_value: &mut FunctionValue,
    symbol_table: &mut HashMap<String, PointerValue<'a>>,
    module: &'a Module,
) {
    let do_while_block_name = function_value.get_unique_block_name();
    let do_while_block = context.append_basic_block(function_value, &do_while_block_name);

    let condition_checker_name = function_value.get_unique_block_name();
    let condition_checker_block_bb =
        context.append_basic_block(function_value, &condition_checker_name);

    let exit_block_name = function_value.get_unique_block_name();
    let exit_block_bb = context.append_basic_block(function_value, &exit_block_name);

    builder.build_unconditional_branch(&do_while_block);
    builder.position_at_end(&do_while_block);

    consume_ast_in_loop(
        block,
        context,
        builder,
        function_value,
        symbol_table,
        &exit_block_bb,
        &condition_checker_block_bb,
        module,
    );
    builder.build_unconditional_branch(&condition_checker_block_bb);

    builder.position_at_end(&condition_checker_block_bb);
    let condition_value = build_expression(
        condition,
        context,
        builder,
        function_value,
        symbol_table,
        module,
        None,
    );

    if let BasicValueEnum::IntValue(cond_value) = condition_value {
        builder.build_conditional_branch(cond_value, &do_while_block, &exit_block_bb);
        builder.position_at_end(&exit_block_bb);
    } else {
        todo!()
    }
}
