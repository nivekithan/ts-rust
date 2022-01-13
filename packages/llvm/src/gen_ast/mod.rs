mod consume_array_member_assignment;
mod consume_do_while_loop;
mod consume_function_declaration;
mod consume_if_block;
mod consume_import_declaration;
mod consume_variable_assignment;
mod consume_variable_declaration;
mod consume_while_loop;

use ast::{declaration::Declaration, Ast};
use inkwell::{
    basic_block::BasicBlock, builder::Builder, context::Context, module::Module,
    values::fn_value::FunctionValue,
};
use lexer::token::KeywordKind;

use crate::{
    gen_ast::{
        consume_array_member_assignment::consume_array_member_assignments,
        consume_do_while_loop::consume_do_while_loop, consume_if_block::consume_if_block,
        consume_variable_assignment::consume_variable_assignment,
        consume_variable_declaration::consume_variable_declaration,
        consume_while_loop::consume_while_loop,
    },
    symbol_table::SymbolTable,
};

use self::{
    consume_function_declaration::consume_function_declaration,
    consume_import_declaration::consume_import_declaration,
};

pub(crate) fn consume_single_ast<'a>(
    ast: &Ast,
    context: &'a Context,
    builder: &'a Builder,
    function_value: &mut FunctionValue,
    symbol_table: &mut SymbolTable<'a>,
    module: &'a Module,
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
                    module,
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
                    module,
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
                    module,
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
                    module,
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
                    module,
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
                    module,
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
    symbol_table: &mut SymbolTable<'a>,
    module: &'a Module,
) {
    for cur_ast in asts.iter() {
        consume_single_ast(
            cur_ast,
            context,
            builder,
            function_value,
            symbol_table,
            module,
        );
    }
}

pub(crate) fn consume_ast_in_loop<'a>(
    asts: &Vec<Ast>,
    context: &'a Context,
    builder: &'a Builder,
    function_value: &mut FunctionValue,
    symbol_table: &mut SymbolTable<'a>,
    exit_block: &BasicBlock,
    continue_block: &BasicBlock,
    module: &'a Module,
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

                _ => {
                    consume_single_ast(ast, context, builder, function_value, symbol_table, module)
                }
            }
        } else {
            todo!()
        }
    }
}

pub(crate) fn consume_ast_in_module<'a>(
    asts: &Vec<Ast>,
    context: &'a Context,
    builder: &'a Builder,
    function_value: &mut FunctionValue,
    symbol_table: &mut SymbolTable<'a>,
    module: &'a Module,
) {
    for cur_ast in asts.iter() {
        if let Ast::Declaration(dec) = cur_ast {
            match dec {
                Declaration::FunctionDeclaration {
                    arguments,
                    blocks,
                    ident_name,
                    return_type,
                } => {
                    consume_function_declaration(
                        arguments,
                        blocks,
                        ident_name,
                        return_type,
                        context,
                        module,
                        symbol_table,
                    );
                }

                Declaration::ImportDeclaration { from: _, ident } => {
                    consume_import_declaration(ident, context, symbol_table, module)
                }

                _ => consume_single_ast(
                    cur_ast,
                    context,
                    builder,
                    function_value,
                    symbol_table,
                    module,
                ),
            }
        }
    }
}
