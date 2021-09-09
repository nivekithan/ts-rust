mod explicit_type_declaration;

use crate::convert_to_ast;
use ast::{
    data_type::DataType,
    expression::{BinaryOperator, Expression, UnaryOperator},
    Ast,
};
use lexer::convert_to_token;
use test_utils::generate_expressions;

#[test]
fn test_float_literal_exp() {
    let input = "
    const x = 1
    ";

    let expected_output: Vec<Ast> = vec![Ast::new_const_variable_declaration(
        &"x".to_string(),
        Expression::FloatLiteralExp {
            name: "1".to_string(),
            value: 1.0,
        },
    )];

    let actual_output = convert_to_ast(convert_to_token(input));

    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_string_literal_exp() {
    let input = "
    const x = \"1\";";

    let expected_output: Vec<Ast> = vec![Ast::new_const_variable_declaration(
        &"x".to_string(),
        Expression::StringLiteralExp {
            value: "1".to_string(),
        },
    )];

    let actual_output = convert_to_ast(convert_to_token(input));

    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_boolean_literal_true_exp() {
    let input = "
    const x = true";

    let expected_output: Vec<Ast> = vec![Ast::new_const_variable_declaration(
        &"x".to_string(),
        Expression::BooleanLiteralExp {
            value: true,
            name: "true".to_string(),
        },
    )];

    let actual_output = convert_to_ast(convert_to_token(input));

    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_boolean_literal_false_exp() {
    let input = "
    const x = false";

    let expected_output: Vec<Ast> = vec![Ast::new_const_variable_declaration(
        &"x".to_string(),
        Expression::BooleanLiteralExp {
            value: false,
            name: "false".to_string(),
        },
    )];

    let actual_output = convert_to_ast(convert_to_token(input));

    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_float_ident_exp() {
    let input = "
    const x = 1;
    const y = x";

    let expected_output: Vec<Ast> = vec![
        Ast::new_const_variable_declaration(
            &"x".to_string(),
            Expression::FloatLiteralExp {
                name: "1".to_string(),
                value: 1.0,
            },
        ),
        Ast::new_const_variable_declaration(
            &"y".to_string(),
            Expression::IdentExp {
                data_type: DataType::Float,
                name: "x".to_string(),
            },
        ),
    ];

    let actual_output = convert_to_ast(convert_to_token(input));

    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_bool_ident_exp() {
    let input = "
    const x = true;
    const y = x";

    let expected_output: Vec<Ast> = vec![
        Ast::new_const_variable_declaration(
            &"x".to_string(),
            Expression::BooleanLiteralExp {
                name: "true".to_string(),
                value: true,
            },
        ),
        Ast::new_const_variable_declaration(
            &"y".to_string(),
            Expression::IdentExp {
                data_type: DataType::Boolean,
                name: "x".to_string(),
            },
        ),
    ];

    let actual_output = convert_to_ast(convert_to_token(input));

    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_string_ident_exp() {
    let input = "
    const x = \"1\";
    const y = x";

    let expected_output: Vec<Ast> = vec![
        Ast::new_const_variable_declaration(
            &"x".to_string(),
            Expression::StringLiteralExp {
                value: "1".to_string(),
            },
        ),
        Ast::new_const_variable_declaration(
            &"y".to_string(),
            Expression::IdentExp {
                data_type: DataType::String,
                name: "x".to_string(),
            },
        ),
    ];

    let actual_output = convert_to_ast(convert_to_token(input));

    assert_eq!(expected_output, actual_output);
}

#[test]
fn test_unary_plus_exp() {
    let expression_forms = generate_expressions(&DataType::Float, "_x");

    for exp_form in expression_forms {
        let input = exp_form.generate_input(format!("const x = +({})", exp_form.main_string));

        let expected_output: Vec<Ast> =
            exp_form.generate_expected_output(vec![Ast::new_const_variable_declaration(
                &"x".to_string(),
                Expression::UnaryExp {
                    operator: UnaryOperator::Plus,
                    argument: Box::new(exp_form.main_exp.clone()),
                },
            )]);

        let actual_output = convert_to_ast(convert_to_token(input.as_str()));

        assert_eq!(expected_output, actual_output)
    }
}

#[test]
fn test_unary_minus_exp() {
    let expression_forms = generate_expressions(&DataType::Float, "_x");

    for exp_form in expression_forms {
        let input = exp_form.generate_input(format!("const x = -({})", exp_form.main_string));

        let expected_output: Vec<Ast> =
            exp_form.generate_expected_output(vec![Ast::new_const_variable_declaration(
                &"x".to_string(),
                Expression::UnaryExp {
                    operator: UnaryOperator::Minus,
                    argument: Box::new(exp_form.main_exp.clone()),
                },
            )]);

        let actual_output = convert_to_ast(convert_to_token(input.as_str()));

        assert_eq!(expected_output, actual_output)
    }
}

#[test]
fn test_unary_bang_exp() {
    let expression_forms = generate_expressions(&DataType::Boolean, "_x");

    for exp_form in expression_forms {
        let input = exp_form.generate_input(format!("const x = !({})", exp_form.main_string));

        let expected_output: Vec<Ast> =
            exp_form.generate_expected_output(vec![Ast::new_const_variable_declaration(
                &"x".to_string(),
                Expression::UnaryExp {
                    operator: UnaryOperator::Bang,
                    argument: Box::new(exp_form.main_exp.clone()),
                },
            )]);

        let actual_output = convert_to_ast(convert_to_token(input.as_str()));

        assert_eq!(expected_output, actual_output)
    }
}

#[test]
fn test_binary_plus_float_exp() {
    let expression_forms_1 = generate_expressions(&DataType::Float, "_x");

    for exp_form_1 in expression_forms_1 {
        let expression_forms_2 = generate_expressions(&DataType::Float, "_y");

        for exp_form_2 in expression_forms_2 {
            let input = exp_form_1.generate_input(exp_form_2.generate_input(format!(
                "
            const x = ({}) + ({})",
                exp_form_1.main_string, exp_form_2.main_string
            )));

            let expected_output =
                exp_form_1.generate_expected_output(exp_form_2.generate_expected_output(vec![
                    Ast::new_const_variable_declaration(
                        &"x".to_string(),
                        Expression::BinaryExp {
                            operator: BinaryOperator::Plus,
                            left: Box::new(exp_form_1.main_exp.clone()),
                            right: Box::new(exp_form_2.main_exp.clone()),
                        },
                    ),
                ]));

            let actual_output = convert_to_ast(convert_to_token(input.as_str()));

            assert_eq!(actual_output, expected_output);
        }
    }
}

#[test]
fn test_binary_plus_string_exp() {
    let expression_forms_1 = generate_expressions(&DataType::String, "_x");

    for exp_form_1 in expression_forms_1 {
        let expression_forms_2 = generate_expressions(&DataType::String, "_y");

        for exp_form_2 in expression_forms_2 {
            let input = exp_form_1.generate_input(exp_form_2.generate_input(format!(
                "
            const x = ({}) + ({})",
                exp_form_1.main_string, exp_form_2.main_string
            )));

            let expected_output =
                exp_form_1.generate_expected_output(exp_form_2.generate_expected_output(vec![
                    Ast::new_const_variable_declaration(
                        &"x".to_string(),
                        Expression::BinaryExp {
                            operator: BinaryOperator::Plus,
                            left: Box::new(exp_form_1.main_exp.clone()),
                            right: Box::new(exp_form_2.main_exp.clone()),
                        },
                    ),
                ]));

            let actual_output = convert_to_ast(convert_to_token(input.as_str()));

            assert_eq!(actual_output, expected_output);
        }
    }
}

#[test]
fn test_binary_minus_exp() {
    let expression_forms_1 = generate_expressions(&DataType::Float, "_x");

    for exp_form_1 in expression_forms_1 {
        let expression_forms_2 = generate_expressions(&DataType::Float, "_y");

        for exp_form_2 in expression_forms_2 {
            let input = exp_form_1.generate_input(exp_form_2.generate_input(format!(
                "
            const x = ({}) - ({})",
                exp_form_1.main_string, exp_form_2.main_string
            )));

            let expected_output =
                exp_form_1.generate_expected_output(exp_form_2.generate_expected_output(vec![
                    Ast::new_const_variable_declaration(
                        &"x".to_string(),
                        Expression::BinaryExp {
                            operator: BinaryOperator::Minus,
                            left: Box::new(exp_form_1.main_exp.clone()),
                            right: Box::new(exp_form_2.main_exp.clone()),
                        },
                    ),
                ]));

            let actual_output = convert_to_ast(convert_to_token(input.as_str()));

            assert_eq!(actual_output, expected_output);
        }
    }
}

#[test]
fn test_binary_star_exp() {
    let expression_forms_1 = generate_expressions(&DataType::Float, "_x");

    for exp_form_1 in expression_forms_1 {
        let expression_forms_2 = generate_expressions(&DataType::Float, "_y");

        for exp_form_2 in expression_forms_2 {
            let input = exp_form_1.generate_input(exp_form_2.generate_input(format!(
                "
            const x = ({}) * ({})",
                exp_form_1.main_string, exp_form_2.main_string
            )));

            let expected_output =
                exp_form_1.generate_expected_output(exp_form_2.generate_expected_output(vec![
                    Ast::new_const_variable_declaration(
                        &"x".to_string(),
                        Expression::BinaryExp {
                            operator: BinaryOperator::Star,
                            left: Box::new(exp_form_1.main_exp.clone()),
                            right: Box::new(exp_form_2.main_exp.clone()),
                        },
                    ),
                ]));

            let actual_output = convert_to_ast(convert_to_token(input.as_str()));

            assert_eq!(actual_output, expected_output);
        }
    }
}

#[test]
fn test_binary_slash_exp() {
    let expression_forms_1 = generate_expressions(&DataType::Float, "_x");

    for exp_form_1 in expression_forms_1 {
        let expression_forms_2 = generate_expressions(&DataType::Float, "_y");

        for exp_form_2 in expression_forms_2 {
            let input = exp_form_1.generate_input(exp_form_2.generate_input(format!(
                "
            const x = ({}) / ({})",
                exp_form_1.main_string, exp_form_2.main_string
            )));

            let expected_output =
                exp_form_1.generate_expected_output(exp_form_2.generate_expected_output(vec![
                    Ast::new_const_variable_declaration(
                        &"x".to_string(),
                        Expression::BinaryExp {
                            operator: BinaryOperator::Slash,
                            left: Box::new(exp_form_1.main_exp.clone()),
                            right: Box::new(exp_form_2.main_exp.clone()),
                        },
                    ),
                ]));

            let actual_output = convert_to_ast(convert_to_token(input.as_str()));

            assert_eq!(actual_output, expected_output);
        }
    }
}

#[test]
fn test_binary_vertical_bar_exp() {
    let expression_forms_1 = generate_expressions(&DataType::Float, "_x");

    for exp_form_1 in expression_forms_1 {
        let expression_forms_2 = generate_expressions(&DataType::Float, "_y");

        for exp_form_2 in expression_forms_2 {
            let input = exp_form_1.generate_input(exp_form_2.generate_input(format!(
                "
            const x = ({}) | ({})",
                exp_form_1.main_string, exp_form_2.main_string
            )));

            let expected_output =
                exp_form_1.generate_expected_output(exp_form_2.generate_expected_output(vec![
                    Ast::new_const_variable_declaration(
                        &"x".to_string(),
                        Expression::BinaryExp {
                            operator: BinaryOperator::VerticalBar,
                            left: Box::new(exp_form_1.main_exp.clone()),
                            right: Box::new(exp_form_2.main_exp.clone()),
                        },
                    ),
                ]));

            let actual_output = convert_to_ast(convert_to_token(input.as_str()));

            assert_eq!(actual_output, expected_output);
        }
    }
}

#[test]
fn test_binary_caret_exp() {
    let expression_forms_1 = generate_expressions(&DataType::Float, "_x");

    for exp_form_1 in expression_forms_1 {
        let expression_forms_2 = generate_expressions(&DataType::Float, "_y");

        for exp_form_2 in expression_forms_2 {
            let input = exp_form_1.generate_input(exp_form_2.generate_input(format!(
                "
            const x = ({}) ^ ({})",
                exp_form_1.main_string, exp_form_2.main_string
            )));

            let expected_output =
                exp_form_1.generate_expected_output(exp_form_2.generate_expected_output(vec![
                    Ast::new_const_variable_declaration(
                        &"x".to_string(),
                        Expression::BinaryExp {
                            operator: BinaryOperator::Caret,
                            left: Box::new(exp_form_1.main_exp.clone()),
                            right: Box::new(exp_form_2.main_exp.clone()),
                        },
                    ),
                ]));

            let actual_output = convert_to_ast(convert_to_token(input.as_str()));

            assert_eq!(actual_output, expected_output);
        }
    }
}

#[test]
fn test_binary_ampersand_exp() {
    let expression_forms_1 = generate_expressions(&DataType::Float, "_x");

    for exp_form_1 in expression_forms_1 {
        let expression_forms_2 = generate_expressions(&DataType::Float, "_y");

        for exp_form_2 in expression_forms_2 {
            let input = exp_form_1.generate_input(exp_form_2.generate_input(format!(
                "
            const x = ({}) & ({})",
                exp_form_1.main_string, exp_form_2.main_string
            )));

            let expected_output =
                exp_form_1.generate_expected_output(exp_form_2.generate_expected_output(vec![
                    Ast::new_const_variable_declaration(
                        &"x".to_string(),
                        Expression::BinaryExp {
                            operator: BinaryOperator::Ampersand,
                            left: Box::new(exp_form_1.main_exp.clone()),
                            right: Box::new(exp_form_2.main_exp.clone()),
                        },
                    ),
                ]));

            let actual_output = convert_to_ast(convert_to_token(input.as_str()));

            assert_eq!(actual_output, expected_output);
        }
    }
}
