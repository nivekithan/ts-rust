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
    let mut context = SymbolContext::new_empty_context();

    while parser.get_cur_token().unwrap() != &Token::Eof {
        let next_ast = parser.next_ast(&mut context);
        asts.push(next_ast);
    }

    return asts;
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use ast::{
        data_type::DataType,
        declaration::{VariableDeclarationKind},
        expression::Expression,
        Ast,
    };
    use indexmap::indexmap;
    use lexer::convert_to_token;

    use crate::convert_to_ast;

    #[test]
    fn test_2() {
        let input = "
        const x = {a : 1, b : 2};
        const y = x.a;";

        let mut exp_hash_map: HashMap<String, Expression> = HashMap::new();

        exp_hash_map.insert(
            "a".to_string(),
            Expression::FloatLiteralExp {
                name: "1".to_string(),
                value: 1.0,
            },
        );
        exp_hash_map.insert(
            "b".to_string(),
            Expression::FloatLiteralExp {
                name: "2".to_string(),
                value: 2.0,
            },
        );

        let expected_output: Vec<Ast> = vec![
            Ast::new_variable_declaration(
                "x_",
                Expression::ObjectLiteral {
                    expression: exp_hash_map,
                    data_type: DataType::ObjectType {
                        entries: indexmap! {"a".to_string() => DataType::Float, "b".to_string() => DataType::Float},
                    },
                },
                VariableDeclarationKind::Const,
            ),
            Ast::new_variable_declaration(
                "y_",
                Expression::DotMemberAccess {
                    container: Box::new(Expression::IdentExp {
                        data_type: DataType::ObjectType {
                            entries: indexmap! {"a".to_string() => DataType::Float, "b".to_string() => DataType::Float},
                        },
                        name: "x_".to_string(),
                    }),
                    argument: "a".to_string(),
                },
                VariableDeclarationKind::Const,
            ),
        ];

        let actual_output = convert_to_ast(convert_to_token(input));

        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test_3() {
        let input = "
        function foo(x : number) : number {
            const x = 1;
        }";

        let expected_output = vec![Ast::new_function_declaration(
            indexmap! {"x".to_string() => DataType::Float},
            Box::new(vec![Ast::new_variable_declaration(
                "x_",
                Expression::FloatLiteralExp {
                    name: "1".to_string(),
                    value: 1.0,
                },
                VariableDeclarationKind::Const,
            )]),
            "foo_".to_string(),
            DataType::Float,
        )];

        let actual_output = convert_to_ast(convert_to_token(input));

        assert_eq!(expected_output, actual_output);
    }
}
