mod consume_if_block;
mod consume_variable_assignment;
mod consume_variable_declaration;

use std::collections::HashMap;

use ast::{declaration::Declaration, Ast};
use inkwell::{
    builder::Builder,
    context::Context,
    values::{fn_value::FunctionValue, ptr_value::PointerValue},
};

use crate::gen_ast::{
    consume_if_block::consume_if_block, consume_variable_assignment::consume_variable_assignment,
    consume_variable_declaration::consume_variable_declaration,
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

                _ => todo!(),
            },

            _ => todo!(),
        }
    }
}
