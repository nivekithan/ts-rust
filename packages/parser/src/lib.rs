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
        declaration::{Declaration, VariableDeclarationKind},
        expression::Expression,
        Ast,
    };
    use lexer::convert_to_token;

    use crate::convert_to_ast;

    #[test]
    fn test_hello() {
        let input = "
        const x = \"1\";
        if (true) {
            const x = 1;
        }";

        let expected_output: Vec<Ast> = vec![
            Ast::Declaration(Declaration::VariableDeclaration {
                ident_name: "x_".to_string(),
                exp: Expression::StringLiteralExp {
                    value: "1".to_string(),
                },
                kind: VariableDeclarationKind::Const,
            }),
            Ast::Declaration(Declaration::IfBlockDeclaration {
                condition: Expression::BooleanLiteralExp {
                    name: "true".to_string(),
                    value: true,
                },
                block: Box::new(vec![Ast::Declaration(Declaration::VariableDeclaration {
                    ident_name: "x_0".to_string(),
                    exp: Expression::FloatLiteralExp {
                        value: 1.0,
                        name: "1".to_string(),
                    },
                    kind: VariableDeclarationKind::Const,
                })]),
            }),
        ];

        let actual_output = convert_to_ast(convert_to_token(input));

        assert_eq!(expected_output, actual_output);
    }
}
