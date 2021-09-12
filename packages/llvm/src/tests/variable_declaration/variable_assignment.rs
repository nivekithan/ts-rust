use ast::data_type::DataType;
use lexer::convert_to_token;
use parser::convert_to_ast;
use test_utils::{DatatypeOrFn, ExpressionTest};

use crate::write_llvm_ir;

#[test]
fn let_float_reassignment() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!(
                "
            let x  = {};
            x = {};
            ",
                main_strings[0], main_strings[1]
            );

            let input = ast_input + main_input.as_str();

            return input;
        }),

        expressions_data_type: vec![
            (
                DatatypeOrFn::DataType(DataType::Float),
                vec!["_x".to_string()],
            ),
            (
                DatatypeOrFn::DataType(DataType::Float),
                vec!["__y".to_string()],
            ),
        ],

        test: Box::new(|_, _, input| {
            let output = write_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}

#[test]
fn let_boolean_reassignment() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!(
                "
            let x  = {};
            x = {};
            ",
                main_strings[0], main_strings[1]
            );

            let input = ast_input + main_input.as_str();

            return input;
        }),

        expressions_data_type: vec![
            (
                DatatypeOrFn::DataType(DataType::Boolean),
                vec!["_x".to_string()],
            ),
            (
                DatatypeOrFn::DataType(DataType::Boolean),
                vec!["__y".to_string()],
            ),
        ],

        test: Box::new(|_, _, input| {
            let output = write_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}

#[test]
fn test_plus_assign() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!(
                "
            let x  = {};
            x += {};
            ",
                main_strings[0], main_strings[1]
            );

            let input = ast_input + main_input.as_str();

            return input;
        }),

        expressions_data_type: vec![
            (
                DatatypeOrFn::DataType(DataType::Float),
                vec!["_x".to_string()],
            ),
            (
                DatatypeOrFn::DataType(DataType::Float),
                vec!["__y".to_string()],
            ),
        ],

        test: Box::new(|_, _, input| {
            let output = write_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}

#[test]
fn test_minus_assign() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!(
                "
            let x  = {};
            x -= {};
            ",
                main_strings[0], main_strings[1]
            );

            let input = ast_input + main_input.as_str();

            return input;
        }),

        expressions_data_type: vec![
            (
                DatatypeOrFn::DataType(DataType::Float),
                vec!["_x".to_string()],
            ),
            (
                DatatypeOrFn::DataType(DataType::Float),
                vec!["__y".to_string()],
            ),
        ],

        test: Box::new(|_, _, input| {
            let output = write_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}

#[test]
fn test_star_assign() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!(
                "
            let x  = {};
            x *= {};
            ",
                main_strings[0], main_strings[1]
            );

            let input = ast_input + main_input.as_str();

            return input;
        }),

        expressions_data_type: vec![
            (
                DatatypeOrFn::DataType(DataType::Float),
                vec!["_x".to_string()],
            ),
            (
                DatatypeOrFn::DataType(DataType::Float),
                vec!["__y".to_string()],
            ),
        ],

        test: Box::new(|_, _, input| {
            let output = write_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}

#[test]
fn test_slash_assign() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!(
                "
            let x  = {};
            x /= {};
            ",
                main_strings[0], main_strings[1]
            );

            let input = ast_input + main_input.as_str();

            return input;
        }),

        expressions_data_type: vec![
            (
                DatatypeOrFn::DataType(DataType::Float),
                vec!["_x".to_string()],
            ),
            (
                DatatypeOrFn::DataType(DataType::Float),
                vec!["__y".to_string()],
            ),
        ],

        test: Box::new(|_, _, input| {
            let output = write_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}
