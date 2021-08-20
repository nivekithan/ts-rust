mod util;

use crate::convert_to_ast;
use ast::{expression::Expression, Ast};
use lexer::convert_to_token;

#[test]
fn test_convert_to_ast() {
    let input = "
        const x = 12;
        const y = 23
        const z =(12);";

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
