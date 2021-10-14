mod parse_block;
mod parse_expression;
mod parse_type_expression;
mod parser;
mod symbol_table;
mod utils;

#[cfg(test)]
mod tests;

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
    use ast::{Ast, data_type::DataType, declaration::{BlockWithCondition, VariableDeclarationKind}, expression::Expression};
    use lexer::convert_to_token;

    use crate::convert_to_ast;

    #[test]
    fn test_2() {
        let input = "
        const x = [1, 1]";

        let expected_output: Vec<Ast> = vec![Ast::new_variable_declaration(
            "x_",
            Expression::ArrayLiteral {
                expression: Box::new(vec![
                    Expression::FloatLiteralExp {
                        name: "1".to_string(),
                        value: 1.0,
                    },
                    Expression::FloatLiteralExp {
                        name: "1".to_string(),
                        value: 1.0,
                    },
                ]),
                expression_data_type : DataType::Float
            },
            VariableDeclarationKind::Const
            
        )];

        let actual_output = convert_to_ast(convert_to_token(input));

        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test_3() {
        let input = "
        do {
        const x = 1;
        } while (true)";

        let expected_output: Vec<Ast> = vec![Ast::new_do_while_loop(BlockWithCondition {
            condition: Expression::BooleanLiteralExp {
                name: "true".to_string(),
                value: true,
            },
            block: Box::new(vec![Ast::new_variable_declaration(
                "x_0",
                Expression::FloatLiteralExp {
                    name: "1".to_string(),
                    value: 1.0,
                },
                VariableDeclarationKind::Const,
            )]),
        })];

        let actual_output = convert_to_ast(convert_to_token(input));

        assert_eq!(expected_output, actual_output);
    }
}
