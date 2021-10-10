use std::collections::HashMap;

use ast::Ast;
use inkwell::{
    builder::Builder,
    context::Context,
    values::{fn_value::FunctionValue, ptr_value::PointerValue},
};

use crate::gen_ast::consume_ast;

pub struct Codegen<'a> {
    content: &'a Vec<Ast>,
    symbol_table: HashMap<String, PointerValue<'a>>,
}

impl<'a> Codegen<'a> {
    pub fn new(content: &Vec<Ast>) -> Codegen {
        return Codegen {
            content,
            symbol_table: HashMap::new(),
        };
    }

    pub(crate) fn consume(
        &mut self,
        context: &'a Context,
        builder: &'a Builder,
        function_value: &mut FunctionValue,
    ) {
        consume_ast(
            self.content,
            context,
            builder,
            function_value,
            &mut self.symbol_table,
        );

        builder.build_return(None);
    }
}
