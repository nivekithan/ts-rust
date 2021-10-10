mod parse_expression;
mod parser;
mod symbol_table;
mod utils;

use crate::{parser::Parser, symbol_table::SymbolContext};
use ast::Ast;
use lexer::token::Token;

pub fn convert_to_ast(input: Vec<Token>) -> Vec<Ast> {
    let mut parser = Parser::new(&input);
    let mut asts: Vec<Ast> = vec![];
    let mut context = SymbolContext::new_global();

    while parser.get_cur_token().unwrap() != &Token::Eof {
        let next_ast = parser.next_ast(&mut context);
        asts.push(next_ast);
    }

    return asts;
}

#[cfg(test)]
mod test {
    use ast::{
        declaration::VariableDeclarationKind,
        expression::{BinaryOperator, Expression},
        Ast,
    };
    use lexer::convert_to_token;

    use crate::convert_to_ast;

    #[test]
    fn test_2() {
        let input = "
        const x = 1 <= 1";

        let expected_output: Vec<Ast> = vec![Ast::new_variable_declaration(
            "x_",
            Expression::BinaryExp {
                operator: BinaryOperator::LessThanOrEqual,
                left: Box::new(Expression::FloatLiteralExp {
                    name: "1".to_string(),
                    value: 1.0,
                }),
                right: Box::new(Expression::FloatLiteralExp {
                    name: "1".to_string(),
                    value: 1.0,
                }),
            },
            VariableDeclarationKind::Const,
        )];

        let actual_output = convert_to_ast(convert_to_token(input));

        assert_eq!(expected_output, actual_output);
    }
}
