use ast::{declaration::BlockWithCondition, Ast};
use lexer::token::Token;

use crate::{parser::Parser, symbol_table::SymbolContext};

impl<'a> Parser<'a> {
    /*
     * Assumes the current token to be '(' in
     *
     *  (<condition>) {
     *      <block>
     *  }
     *
     * Consumes token till `}` in
     *
     *  (<condition>) {
     *      <block>
     *  }
     *
     * Pass Current scope context no need to create child context
     *
     * */
    pub(crate) fn parse_block_with_condition(
        &mut self,
        context: &mut SymbolContext,
    ) -> Result<BlockWithCondition, String> {
        self.assert_cur_token(&Token::CurveOpenBracket)?;
        self.next(); // consumes (

        let condition = self.parse_expression(1, context)?;

        self.assert_cur_token(&Token::CurveCloseBracket)?;
        self.next(); // consumes )

        let ast_block = self.parse_block(context)?;

        let block_with_condition = BlockWithCondition::new(condition, ast_block);
        return Ok(block_with_condition);
    }

    /*
     * Assumes the current token to be `{` in
     *
     *  {
     *      <block>
     *  }
     *
     * Consumes till token } in
     *
     * {
     *     <block>
     * }
     *
     * Pass current scope context no need to create child context
     *
     * */
    pub(crate) fn parse_block(&mut self, context: &mut SymbolContext) -> Result<Vec<Ast>, String> {
        self.assert_cur_token(&Token::AngleOpenBracket)?;
        self.next(); // consumes {

        let cur_value = context.counter;
        let suffix = format!("{}{}", context.suffix, cur_value);
        context.counter += 1;

        let mut child_context = context.create_child_context(suffix);

        let mut ast_block: Vec<Ast> = vec![];

        while self.get_cur_token().unwrap() != &Token::AngleCloseBracket {
            let ast = self.next_ast_in_context(&mut child_context)?;
            ast_block.push(ast);
        }

        self.next(); // consumes }

        return Ok(ast_block);
    }
}
