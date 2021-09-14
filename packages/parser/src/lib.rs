mod parser;
mod symbol_table;

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
        declaration::{Declaration, VariableAssignmentOperator, VariableDeclarationKind},
        expression::{BinaryOperator, Expression},
        Ast,
    };
    use lexer::convert_to_token;

    use crate::convert_to_ast;

    #[test]
    fn test_hello() {
        let input = "
        let x = 1;
        if (true) {
             x = 10;
             let x = true;
             x = false;
        }";

        let expected_output: Vec<Ast> = vec![
            Ast::Declaration(Declaration::VariableDeclaration {
                ident_name: "x_".to_string(),
                exp: Expression::FloatLiteralExp {
                    value: 1.0,
                    name: "1".to_string(),
                },
                kind: VariableDeclarationKind::Let,
            }),
            Ast::Declaration(Declaration::IfBlockDeclaration {
                condition: Expression::BooleanLiteralExp {
                    name: "true".to_string(),
                    value: true,
                },
                block: Box::new(vec![
                    Ast::Declaration(Declaration::VariableAssignment {
                        ident_name: "x_".to_string(),
                        exp: Expression::FloatLiteralExp {
                            value: 10.0,
                            name: "10".to_string(),
                        },
                        operator: VariableAssignmentOperator::Assign,
                    }),
                    Ast::Declaration(Declaration::VariableDeclaration {
                        ident_name: "x_0".to_string(),
                        exp: Expression::BooleanLiteralExp {
                            name: "true".to_string(),
                            value: true,
                        },
                        kind: VariableDeclarationKind::Let,
                    }),
                    Ast::Declaration(Declaration::VariableAssignment {
                        ident_name: "x_0".to_string(),
                        exp: Expression::BooleanLiteralExp {
                            name: "false".to_string(),
                            value: false,
                        },
                        operator: VariableAssignmentOperator::Assign,
                    }),
                ]),
            }),
        ];

        let actual_output = convert_to_ast(convert_to_token(input));

        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test_2() {
        let input = "
        const x = 1 === 1";

        let expected_output: Vec<Ast> = vec![Ast::new_variable_declaration(
            "x_",
            Expression::BinaryExp {
                operator: BinaryOperator::StrictEquality,
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
