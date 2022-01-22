mod parse_block;
mod parse_expression;
mod parse_type_expression;
mod parser;
pub mod symbol_table;
pub mod traits;
mod utils;

#[cfg(test)]
mod tests;

use std::collections::HashMap;

use crate::{parser::Parser, symbol_table::SymbolContext};
use ast::Ast;
use lexer::token::Token;
use symbol_table::SymbolMetaInsert;
use traits::{DummyImportResolver, ImportResolver};

pub fn convert_to_ast(input: Vec<Token>) -> Vec<Ast> {
    let resolver = &mut DummyImportResolver::new();
    return consume_token(input, resolver, None).0;
}

pub fn consume_token<'a, R: ImportResolver>(
    input: Vec<Token>,
    resolver: &mut R,
    file_name: Option<&str>,
) -> (Vec<Ast>, HashMap<String, SymbolMetaInsert>) {
    let mut parser = Parser::new(&input, resolver, file_name);
    let mut asts: Vec<Ast> = vec![];
    let mut context = SymbolContext::create_global_context();

    while parser.get_cur_token().unwrap() != &Token::Eof {
        let next_ast = parser.next_ast(&mut context);
        asts.push(next_ast);
    }

    return (asts, context.global_symbols);
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use ast::{
        data_type::DataType, declaration::VariableDeclarationKind, expression::Expression, Ast,
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
                "x|_|",
                Expression::ObjectLiteral {
                    expression: exp_hash_map,
                    data_type: DataType::ObjectType {
                        entries: indexmap! {"a".to_string() => DataType::Float, "b".to_string() => DataType::Float},
                    },
                },
                VariableDeclarationKind::Const,
            ),
            Ast::new_variable_declaration(
                "y|_|",
                Expression::DotMemberAccess {
                    container: Box::new(Expression::IdentExp {
                        data_type: DataType::ObjectType {
                            entries: indexmap! {"a".to_string() => DataType::Float, "b".to_string() => DataType::Float},
                        },
                        name: "x|_|".to_string(),
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
    fn test_4() {
        let input = "
        function foo(x : number) : number {
           return 1;
        }
        
        const y = foo(4);";

        let expected_output = vec![
            Ast::new_function_declaration(
                indexmap! {"x|_|".to_string() => DataType::Float},
                Box::new(vec![Ast::new_return_statement(Some(
                    Expression::FloatLiteralExp {
                        name: "1".to_string(),
                        value: 1.0,
                    },
                ))]),
                "foo|_|".to_string(),
                DataType::Float,
            ),
            Ast::new_variable_declaration(
                "y|_|",
                Expression::FunctionCall {
                    fn_name: "foo|_|".to_string(),
                    parameters: vec![Expression::FloatLiteralExp {
                        name: "4".to_string(),
                        value: 4.0,
                    }],
                    return_type: DataType::Float,
                },
                VariableDeclarationKind::Const,
            ),
        ];

        let actual_output = convert_to_ast(convert_to_token(input));

        assert_eq!(expected_output, actual_output);
    }
}
