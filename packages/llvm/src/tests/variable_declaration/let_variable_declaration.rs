use ast::data_type::DataType;
use lexer::convert_to_token;
use parser::convert_to_ast;
use test_utils::{DatatypeOrFn, ExpressionTest};

use crate::compile_to_llvm_ir;

#[test]
fn test_all_let() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!("let x  = {}", main_strings[0]);

            let input = ast_input + main_input.as_str();

            return input;
        }),

        expressions_data_type: vec![(
            DatatypeOrFn::Fn(Box::new(|_| {
                return true;
            })),
            vec!["_x".to_string()],
        )],

        test: Box::new(|_, _, input| {
            let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}

#[test]
fn explicit_float_type_let() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!("let x : number = {}", main_strings[0]);

            let input = ast_input + main_input.as_str();

            return input;
        }),

        expressions_data_type: vec![(
            DatatypeOrFn::DataType(DataType::Float),
            vec!["_x".to_string()],
        )],

        test: Box::new(|_, _, input| {
            let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}

#[test]
fn explicit_boolean_type_let() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!("let x : boolean = {}", main_strings[0]);

            let input = ast_input + main_input.as_str();

            return input;
        }),

        expressions_data_type: vec![(
            DatatypeOrFn::DataType(DataType::Boolean),
            vec!["_x".to_string()],
        )],

        test: Box::new(|_, _, input| {
            let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}

#[test]
fn test_let_unary_bang() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!("let x  = !({})", main_strings[0]);

            let input = ast_input + main_input.as_str();

            return input;
        }),

        expressions_data_type: vec![(
            DatatypeOrFn::DataType(DataType::Boolean),
            vec!["_x".to_string()],
        )],

        test: Box::new(|_, _, input| {
            let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}

#[test]
fn unary_plus() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!("let x  = +({})", main_strings[0]);

            let input = ast_input + main_input.as_str();

            return input;
        }),

        expressions_data_type: vec![(
            DatatypeOrFn::DataType(DataType::Float),
            vec!["_x".to_string()],
        )],

        test: Box::new(|_, _, input| {
            let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}

#[test]
fn unary_minus() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!("let x  = -({})", main_strings[0]);

            let input = ast_input + main_input.as_str();

            return input;
        }),

        expressions_data_type: vec![(
            DatatypeOrFn::DataType(DataType::Float),
            vec!["_x".to_string()],
        )],

        test: Box::new(|_, _, input| {
            let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}

#[test]
fn binary_float_plus() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!("let x  = ({}) + ({})", main_strings[0], main_strings[1]);

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
            let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}

#[test]
fn binary_float_minus() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!("let x  = ({}) - ({})", main_strings[0], main_strings[1]);

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
            let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}

#[test]
fn binary_float_star() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!("let x  = ({}) * ({})", main_strings[0], main_strings[1]);

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
            let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}

#[test]
fn binary_float_slash() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!("let x  = ({}) / ({})", main_strings[0], main_strings[1]);

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
            let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}

#[test]
fn binary_strict_equality_bool() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!("let x  = ({}) === ({})", main_strings[0], main_strings[1]);

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
            let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}

#[test]
fn binary_strict_equality_float() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!("let x  = ({}) === ({})", main_strings[0], main_strings[1]);

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
            let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}

#[test]
fn binary_strict_not_equal_bool() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!("let x  = ({}) !== ({})", main_strings[0], main_strings[1]);

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
            let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}
#[test]
fn binary_strict_not_equal_float() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!("let x  = ({}) !== ({})", main_strings[0], main_strings[1]);

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
            let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}

#[test]
fn binary_less_than() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!("let x  = ({}) < ({})", main_strings[0], main_strings[1]);

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
            let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}

#[test]
fn binary_less_than_or_equal() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!("let x  = ({}) <= ({})", main_strings[0], main_strings[1]);

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
            let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}

#[test]
fn binary_greater_than() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!("let x  = ({}) > ({})", main_strings[0], main_strings[1]);

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
            let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}

#[test]
fn binary_greater_than_or_equal() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!("let x  = ({}) >= ({})", main_strings[0], main_strings[1]);

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
            let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test();
}
