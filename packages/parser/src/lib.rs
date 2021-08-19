pub mod parser;

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

#[cfg(test)]
mod test {
    use ast::{expression::Expression, Ast};
    use lexer::convert_to_token;

    use crate::convert_to_ast;

    #[test]
    fn test_convert_to_ast() {
        let input = "
        const x = 12;
        const y = 23
        const z =12;";

        let expected_output = vec![
            Ast::new_const_variable_declaration(
                &"x".to_string(),
                Expression::FloatLiteralExp {
                    name: "12".to_string(),
                    value: 12.0,
                },
            ),
            Ast::new_const_variable_declaration(
                &"y".to_string(),
                Expression::FloatLiteralExp {
                    name: "23".to_string(),
                    value: 23.0,
                },
            ),
            Ast::new_const_variable_declaration(
                &"z".to_string(),
                Expression::FloatLiteralExp {
                    name: "12".to_string(),
                    value: 12.0,
                },
            ),
        ];

        let actual_output = convert_to_ast(convert_to_token(input));

        assert_eq!(actual_output, expected_output);
    }
}
