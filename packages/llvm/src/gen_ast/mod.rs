mod consume_array_member_assignment;
mod consume_do_while_loop;
mod consume_if_block;
mod consume_variable_assignment;
mod consume_variable_declaration;
mod consume_while_loop;
use std::collections::HashMap;

use ast::{declaration::Declaration, Ast};
use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    values::{fn_value::FunctionValue, ptr_value::PointerValue},
};
use lexer::token::KeywordKind;

use crate::gen_ast::{
    consume_array_member_assignment::consume_array_member_assignments,
    consume_do_while_loop::consume_do_while_loop, consume_if_block::consume_if_block,
    consume_variable_assignment::consume_variable_assignment,
    consume_variable_declaration::consume_variable_declaration,
    consume_while_loop::consume_while_loop,
};

pub(crate) fn consume_single_ast<'a>(
    ast: &Ast,
    context: &'a Context,
    builder: &'a Builder,
    function_value: &mut FunctionValue,
    symbol_table: &mut HashMap<String, PointerValue<'a>>,
) {
    match ast {
        Ast::Declaration(dec) => match dec {
            Declaration::VariableDeclaration {
                ident_name,
                exp,
                kind: _,
            } => {
                consume_variable_declaration(
                    ident_name,
                    exp,
                    context,
                    builder,
                    function_value,
                    symbol_table,
                );
            }

            // Does not do typechecking
            Declaration::VariableAssignment {
                ident_name,
                exp,
                operator,
            } => {
                consume_variable_assignment(
                    ident_name,
                    exp,
                    operator,
                    context,
                    builder,
                    function_value,
                    symbol_table,
                );
            }

            Declaration::NewIfBlockDeclaration {
                else_block,
                else_if_block,
                if_block,
            } => {
                consume_if_block(
                    if_block,
                    else_if_block,
                    else_block,
                    context,
                    builder,
                    function_value,
                    symbol_table,
                );
            }

            Declaration::WhileLoopDeclaration { block, condition } => {
                consume_while_loop(
                    block,
                    condition,
                    context,
                    builder,
                    function_value,
                    symbol_table,
                );
            }

            Declaration::DoWhileLoopDeclaration { block, condition } => {
                consume_do_while_loop(
                    block,
                    condition,
                    context,
                    builder,
                    function_value,
                    symbol_table,
                );
            }

            Declaration::ArrayMemberAssignment {
                ident_name,
                member_access_exp,
                operator,
                exp,
            } => {
                consume_array_member_assignments(
                    ident_name,
                    member_access_exp,
                    operator,
                    exp,
                    context,
                    builder,
                    function_value,
                    symbol_table,
                );
            }

            _ => panic!("Unknown declaration {:?}", dec),
        },

        _ => todo!(),
    }
}

pub(crate) fn consume_generic_ast<'a>(
    asts: &Vec<Ast>,
    context: &'a Context,
    builder: &'a Builder,
    function_value: &mut FunctionValue,
    symbol_table: &mut HashMap<String, PointerValue<'a>>,
) {
    for cur_ast in asts.iter() {
        consume_single_ast(cur_ast, context, builder, function_value, symbol_table);
    }
}

pub(crate) fn consume_ast_in_loop<'a>(
    asts: &Vec<Ast>,
    context: &'a Context,
    builder: &'a Builder,
    function_value: &mut FunctionValue,
    symbol_table: &mut HashMap<String, PointerValue<'a>>,
    exit_block: &BasicBlock,
    continue_block: &BasicBlock,
) {
    for ast in asts.iter() {
        if let Ast::Declaration(dec) = ast {
            match dec {
                Declaration::LoopControlFlow { keyword } => match keyword {
                    KeywordKind::Break => {
                        builder.build_unconditional_branch(exit_block);
                    }
                    KeywordKind::Continue => {
                        builder.build_unconditional_branch(continue_block);
                    }

                    _ => panic!("Unexpected keyword {:?}", keyword),
                },

                _ => consume_single_ast(ast, context, builder, function_value, symbol_table),
            }
        } else {
            todo!()
        }
    }
}
