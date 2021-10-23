use ast::data_type::DataType;
use lexer::convert_to_token;
use parser::convert_to_ast;
use test_utils::{DatatypeOrFn, ExpressionTest};

use crate::write_llvm_ir;


#[test]
fn test_array_member_assign() {
    let exp_test = ExpressionTest {
        generate_input: Box::new(|ast_strings, main_strings| {
            let ast_input = "\n".to_string() + ast_strings.join("\n").as_str();

            let main_input = format!(
                "
            const x  = {};
            x[1] = {};
            ",
                main_strings[0], main_strings[1]
            );

            let input = ast_input + main_input.as_str();

            return input;
        }),

        expressions_data_type: vec![
            (
                DatatypeOrFn::DataType(DataType::ArrayType{base_type : Box::new(DataType::Float)}),
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