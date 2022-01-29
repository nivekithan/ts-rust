use ast::AstPtr;
use inkwell::{
    builder::Builder, context::Context, module::Module, values::fn_value::FunctionValue,
};

use crate::{gen_ast::consume_ast_in_module, symbol_table::SymbolTable};

pub struct Codegen<'a> {
    content: &'a Vec<AstPtr>,
    symbol_table: SymbolTable<'a>,
}

impl<'a> Codegen<'a> {
    pub fn new(content: &Vec<AstPtr>) -> Codegen {
        return Codegen {
            content,
            symbol_table: SymbolTable::new(),
        };
    }

    pub(crate) fn consume(
        &mut self,
        context: &'a Context,
        builder: &'a Builder,
        module: &'a Module,
        function_value: &mut FunctionValue,
    ) {
        consume_ast_in_module(
            self.content,
            context,
            builder,
            function_value,
            &mut self.symbol_table,
            module,
        );
    }
}
