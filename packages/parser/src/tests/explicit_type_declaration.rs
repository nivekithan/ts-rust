use ast::{data_type::DataType, declaration::VariableDeclarationKind, Ast};
use lexer::convert_to_token;
use test_utils::expression_form::generate_expressions;

use crate::convert_to_ast;

#[test]
fn string_type_declaration() {
    let expressions_form = generate_expressions(&DataType::String, "_x");

    for exp_form in expressions_form.iter() {
        let input = exp_form.generate_input(format!("const x : string = {}", exp_form.main_string));
        let expected_output =
            exp_form.generate_expected_output(vec![Ast::new_variable_declaration(
                &"x".to_string(),
                exp_form.main_exp.clone(),
                VariableDeclarationKind::Const,
            )]);

        let actual_output = convert_to_ast(convert_to_token(input.as_str()));

        assert_eq!(expected_output, actual_output);
    }
}

#[test]
fn number_type_declaration() {
    let expressions_form = generate_expressions(&DataType::Float, "_x");

    for exp_form in expressions_form.iter() {
        let input = exp_form.generate_input(format!("const x : number = {}", exp_form.main_string));
        let expected_output =
            exp_form.generate_expected_output(vec![Ast::new_variable_declaration(
                &"x".to_string(),
                exp_form.main_exp.clone(),
                VariableDeclarationKind::Const,
            )]);

        let actual_output = convert_to_ast(convert_to_token(input.as_str()));

        assert_eq!(expected_output, actual_output);
    }
}

#[test]
fn boolean_type_declaration() {
    let expressions_form = generate_expressions(&DataType::Boolean, "_x");

    for exp_form in expressions_form.iter() {
        let input =
            exp_form.generate_input(format!("const x : boolean = {}", exp_form.main_string));
        let expected_output =
            exp_form.generate_expected_output(vec![Ast::new_variable_declaration(
                &"x".to_string(),
                exp_form.main_exp.clone(),
                VariableDeclarationKind::Const,
            )]);

        let actual_output = convert_to_ast(convert_to_token(input.as_str()));

        assert_eq!(expected_output, actual_output);
    }
}
