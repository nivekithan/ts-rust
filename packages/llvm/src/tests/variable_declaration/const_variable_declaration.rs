use ast::data_type::DataType;
use lexer::convert_to_token;
use parser::convert_to_ast;

use crate::{
    tests::utils::{DatatypeOrFn, ExpressionTest},
    write_llvm_ir,
};



#[test]
fn test_all_const() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
        
            let ast_input = "\n".to_string() +  ast_strings.join("\n").as_str();

            let main_input = format!("const x  = {}", main_strings[0]);

            let input = ast_input + main_input.as_str();

            return input;
        }),

        expressions_data_type: vec![DatatypeOrFn::Fn(Box::new(|_| {
            return true;
        }))],

        test: Box::new(|_, _, input| {
            let output = write_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test(vec!["_x".to_string()]);
}

#[test]
fn test_const_unary_bang() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() +  ast_strings.join("\n").as_str();

            let main_input = format!("const x  = !({})", main_strings[0]);

            let input = ast_input + main_input.as_str();

            return input;
        }),

        expressions_data_type: vec![DatatypeOrFn::DataType(DataType::Boolean)],

        test: Box::new(|_, _, input| {
            let output = write_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, output);
        }),
    };

    exp_test.test(vec!["_x".to_string()]);
}
