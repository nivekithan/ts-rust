use ast::{
    data_type::DataType,
    expression::{BinaryOperator, Expression},
};
use lexer::convert_to_token;
use parser::convert_to_ast;
use test_utils::{generate_expressions, ExpressionForm};

use crate::write_llvm_ir;

#[test]
fn test_float_literal_exp() {
    let input = "
    const x = 1
    ";

    let actual_output = write_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(actual_output, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    define void @main() {
    entry:
      %x = alloca double, align 8
      store double 1.000000e+00, double* %x, align 8
      ret void
    }
    "###);
}

// #[test]
// fn test_string_literal_exp() {
//     let input = "
//     const x = \"1\";";

//     let actual_output = write_llvm_ir(convert_to_ast(convert_to_token(input)));

//     insta::assert_snapshot!(actual_output, @"");
// }

#[test]
fn test_boolean_literal_true_exp() {
    let input = "
    const x = true";

    let actual_output = write_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(actual_output, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    define void @main() {
    entry:
      %x = alloca i64, align 8
      store i64 1, i64* %x, align 4
      ret void
    }
    "###);
}

#[test]
fn test_boolean_literal_false_exp() {
    let input = "
    const x = false";

    let actual_output = write_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(actual_output, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    define void @main() {
    entry:
      %x = alloca i64, align 8
      store i64 0, i64* %x, align 4
      ret void
    }
    "###);
}

#[test]
fn test_float_ident_exp() {
    let input = "
    const x = 1;
    const y = x";

    let actual_output = write_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(actual_output, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    define void @main() {
    entry:
      %x = alloca double, align 8
      store double 1.000000e+00, double* %x, align 8
      %y = alloca double, align 8
      %"1" = load double, double* %x, align 8
      store double %"1", double* %y, align 8
      ret void
    }
    "###);
}

#[test]
fn test_bool_ident_exp() {
    let input = "
    const x = true;
    const y = x";

    let actual_output = write_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(actual_output, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    define void @main() {
    entry:
      %x = alloca i64, align 8
      store i64 1, i64* %x, align 4
      %y = alloca i64, align 8
      %"1" = load i64, i64* %x, align 4
      store i64 %"1", i64* %y, align 4
      ret void
    }
    "###);
}

// #[test]
// fn test_string_ident_exp() {
//     let input = "
//     const x = \"1\";
//     const y = x";

//     let actual_output = write_llvm_ir(convert_to_ast(convert_to_token(input)));

//     insta::assert_snapshot!(actual_output, @"");
// }

#[test]
fn test_unary_plus_exp() {
    let expression_forms = generate_expressions(&DataType::Float, "_x");

    for exp_form in expression_forms {
        if is_working_in_progress(&exp_form) {
            continue;
        }

        println!("{:?}", exp_form);

        let input = exp_form.generate_input(format!("const x = +({})", exp_form.main_string));

        let actual_output = write_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

        insta::assert_snapshot!(input, actual_output);
    }
}

#[test]
fn test_unary_minus_exp() {
    let expression_forms = generate_expressions(&DataType::Float, "_x");

    for exp_form in expression_forms {
        if is_working_in_progress(&exp_form) {
            continue;
        }

        let input = exp_form.generate_input(format!("const x = -({})", exp_form.main_string));

        let actual_output = write_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

        insta::assert_snapshot!(input, actual_output);
    }
}

#[test]
fn test_unary_bang_exp() {
    let expression_forms = generate_expressions(&DataType::Boolean, "_x");

    for exp_form in expression_forms {
        if is_working_in_progress(&exp_form) {
            continue;
        }

        let input = exp_form.generate_input(format!("const x = !({})", exp_form.main_string));

        let actual_output = write_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

        insta::assert_snapshot!(input, actual_output);
    }
}

#[test]
fn test_binary_plus_float_exp() {
    let expression_forms_1 = generate_expressions(&DataType::Float, "_x");

    for exp_form_1 in expression_forms_1 {
        let expression_forms_2 = generate_expressions(&DataType::Float, "_y");

        if is_working_in_progress(&exp_form_1) {
            continue;
        }

        for exp_form_2 in expression_forms_2 {

            if  is_working_in_progress(&exp_form_2) {
                continue;
            }

            let input = exp_form_1.generate_input(exp_form_2.generate_input(format!(
                "
            const x = ({}) + ({})",
                exp_form_1.main_string, exp_form_2.main_string
            )));

            let actual_output = write_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, actual_output);
        }
    }
}

// #[test]
// fn test_binary_plus_string_exp() {
//     let expression_forms_1 = generate_expressions(&DataType::String, "_x");

//     for exp_form_1 in expression_forms_1 {
//         let expression_forms_2 = generate_expressions(&DataType::String, "_y");

//         for exp_form_2 in expression_forms_2 {

//             if is_working_in_progress(&exp_form_1) || is_working_in_progress(&exp_form_2) {
//                 continue;
//             }

//             let input = exp_form_1.generate_input(exp_form_2.generate_input(format!(
//                 "
//             const x = ({}) + ({})",
//                 exp_form_1.main_string, exp_form_2.main_string
//             )));

//             let actual_output = write_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

//             insta::assert_snapshot!(actual_output, @"");
//         }
//     }
// }

#[test]
fn test_binary_minus_exp() {
    let expression_forms_1 = generate_expressions(&DataType::Float, "_x");

    for exp_form_1 in expression_forms_1 {
        let expression_forms_2 = generate_expressions(&DataType::Float, "_y");

        if is_working_in_progress(&exp_form_1) {
            continue;
        }

        for exp_form_2 in expression_forms_2 {

            if  is_working_in_progress(&exp_form_2) {
                continue;
            }

            let input = exp_form_1.generate_input(exp_form_2.generate_input(format!(
                "
            const x = ({}) - ({})",
                exp_form_1.main_string, exp_form_2.main_string
            )));

            let actual_output = write_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, actual_output);
        }
    }
}

#[test]
fn test_binary_star_exp() {
    let expression_forms_1 = generate_expressions(&DataType::Float, "_x");

    for exp_form_1 in expression_forms_1 {
        let expression_forms_2 = generate_expressions(&DataType::Float, "_y");

        if is_working_in_progress(&exp_form_1) {
            continue;
        }

        for exp_form_2 in expression_forms_2 {

            if  is_working_in_progress(&exp_form_2) {
                continue;
            }

            let input = exp_form_1.generate_input(exp_form_2.generate_input(format!(
                "
            const x = ({}) * ({})",
                exp_form_1.main_string, exp_form_2.main_string
            )));

            let actual_output = write_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, actual_output);
        }
    }
}

#[test]
fn test_binary_slash_exp() {
    let expression_forms_1 = generate_expressions(&DataType::Float, "_x");

    for exp_form_1 in expression_forms_1 {
        let expression_forms_2 = generate_expressions(&DataType::Float, "_y");

        if is_working_in_progress(&exp_form_1) {
            continue;
        }

        for exp_form_2 in expression_forms_2 {

            if  is_working_in_progress(&exp_form_2) {
                continue;
            }

            let input = exp_form_1.generate_input(exp_form_2.generate_input(format!(
                "
            const x = ({}) / ({})",
                exp_form_1.main_string, exp_form_2.main_string
            )));

            let actual_output = write_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

            insta::assert_snapshot!(input, actual_output);
        }
    }
}

// #[test]
// fn test_binary_vertical_bar_exp() {
//     let expression_forms_1 = generate_expressions(&DataType::Float, "_x");

//     for exp_form_1 in expression_forms_1 {
//         let expression_forms_2 = generate_expressions(&DataType::Float, "_y");

//         for exp_form_2 in expression_forms_2 {

//             if is_working_in_progress(&exp_form_1) || is_working_in_progress(&exp_form_2) {
//                 continue;
//             }

//             let input = exp_form_1.generate_input(exp_form_2.generate_input(format!(
//                 "
//             const x = ({}) | ({})",
//                 exp_form_1.main_string, exp_form_2.main_string
//             )));

//             let actual_output = write_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

//             insta::assert_snapshot!(actual_output, @"");
//         }
//     }
// }

// #[test]
// fn test_binary_caret_exp() {
//     let expression_forms_1 = generate_expressions(&DataType::Float, "_x");

//     for exp_form_1 in expression_forms_1 {
//         let expression_forms_2 = generate_expressions(&DataType::Float, "_y");

//         for exp_form_2 in expression_forms_2 {

//             if is_working_in_progress(&exp_form_1) || is_working_in_progress(&exp_form_2) {
//                 continue;
//             }

//             let input = exp_form_1.generate_input(exp_form_2.generate_input(format!(
//                 "
//             const x = ({}) ^ ({})",
//                 exp_form_1.main_string, exp_form_2.main_string
//             )));

//             let actual_output = write_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

//             insta::assert_snapshot!(actual_output, @"");
//         }
//     }
// }

// #[test]
// fn test_binary_ampersand_exp() {
//     let expression_forms_1 = generate_expressions(&DataType::Float, "_x");

//     for exp_form_1 in expression_forms_1 {
//         let expression_forms_2 = generate_expressions(&DataType::Float, "_y");

//         for exp_form_2 in expression_forms_2 {

//             if is_working_in_progress(&exp_form_1) || is_working_in_progress(&exp_form_2) {
//                 continue;
//             }

//             let input = exp_form_1.generate_input(exp_form_2.generate_input(format!(
//                 "
//             const x = ({}) & ({})",
//                 exp_form_1.main_string, exp_form_2.main_string
//             )));

//             let actual_output = write_llvm_ir(convert_to_ast(convert_to_token(input.as_str())));

//             insta::assert_snapshot!(actual_output, @"");
//         }
//     }
// }

fn is_working_in_progress(exp_form: &ExpressionForm) -> bool {
    let final_expression = &exp_form.main_exp;

    let result = match final_expression {
        Expression::BinaryExp {
            operator,
            left: _,
            right: _,
        } => match operator {
            BinaryOperator::Ampersand | BinaryOperator::Caret | BinaryOperator::VerticalBar => true,
            _ =>  false,
        },

        Expression::UnaryExp {
            operator: _,
            argument: _,
        } => return false,

        Expression::FloatLiteralExp { name: _, value: _ } => return true,
        Expression::BooleanLiteralExp { name: _, value: _ } => return false,

        Expression::StringLiteralExp { value: _ } => true,

        Expression::IdentExp { name: _, data_type } => match data_type {
            DataType::String => true,

            _ => false,
        },
    };

    if !result {
        println!("Skipping expression {:?}", final_expression);
    } else {
        println!("Accepting expression {:?}", final_expression);
    }

    return result;
}
