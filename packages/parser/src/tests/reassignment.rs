use ast::{Ast, data_type::DataType, declaration::VariableDeclarationKind};
use lexer::convert_to_token;
use test_utils::expression_form::generate_expressions;

use crate::convert_to_ast;

#[test]
fn string_reassign() {
    let expression_form_1 = generate_expressions(&DataType::String, "_x");

    for exp_form_1 in expression_form_1 {
        let expression_form_2 = generate_expressions(&DataType::String, "_y");

        for exp_form_2 in expression_form_2 {
            let input = exp_form_1.generate_input(exp_form_2.generate_input(format!(
                "let x = {}; \n x = {}",
                exp_form_1.main_string, exp_form_2.main_string
            )));

            let expected_output = exp_form_1.generate_expected_output(exp_form_2.generate_expected_output(vec![
                Ast::new_variable_declaration("x", exp_form_1.main_exp.clone(), VariableDeclarationKind::Let),
                Ast::new_reassignment("x", exp_form_2.main_exp.clone())
            ]));

            let actual_output = convert_to_ast(convert_to_token(input.as_str()));

            assert_eq!(expected_output, actual_output);
        }
    }
}



#[test]
fn boolean_reassign() {
    let expression_form_1 = generate_expressions(&DataType::Boolean, "_x");

    for exp_form_1 in expression_form_1 {
        let expression_form_2 = generate_expressions(&DataType::Boolean, "_y");

        for exp_form_2 in expression_form_2 {
            let input = exp_form_1.generate_input(exp_form_2.generate_input(format!(
                "let x = {}; \n x = {}",
                exp_form_1.main_string, exp_form_2.main_string
            )));

            let expected_output = exp_form_1.generate_expected_output(exp_form_2.generate_expected_output(vec![
                Ast::new_variable_declaration("x", exp_form_1.main_exp.clone(), VariableDeclarationKind::Let),
                Ast::new_reassignment("x", exp_form_2.main_exp.clone())
            ]));

            let actual_output = convert_to_ast(convert_to_token(input.as_str()));

            assert_eq!(expected_output, actual_output);
        }
    }
}

#[test]
fn float_reassign() {
    let expression_form_1 = generate_expressions(&DataType::Float, "_x");

    for exp_form_1 in expression_form_1 {
        let expression_form_2 = generate_expressions(&DataType::Float, "_y");

        for exp_form_2 in expression_form_2 {
            let input = exp_form_1.generate_input(exp_form_2.generate_input(format!(
                "let x = {}; \n x = {}",
                exp_form_1.main_string, exp_form_2.main_string
            )));

            let expected_output = exp_form_1.generate_expected_output(exp_form_2.generate_expected_output(vec![
                Ast::new_variable_declaration("x", exp_form_1.main_exp.clone(), VariableDeclarationKind::Let),
                Ast::new_reassignment("x", exp_form_2.main_exp.clone())
            ]));

            let actual_output = convert_to_ast(convert_to_token(input.as_str()));

            assert_eq!(expected_output, actual_output);
        }
    }
}
