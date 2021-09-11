mod parser;
mod symbol_table;

use crate::parser::Parser;
use ast::Ast;
use lexer::token::Token;

pub fn convert_to_ast(input: Vec<Token>) -> Vec<Ast> {
    let mut parser = Parser::new(&input);
    let mut asts: Vec<Ast> = vec![];

    while parser.get_cur_token().unwrap() != &Token::Eof {
        let next_ast = parser.next_ast();
        asts.push(next_ast);
    }

    return asts;
}
