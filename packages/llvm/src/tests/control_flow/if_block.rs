use lexer::convert_to_token;
use parser::convert_to_ast;

use crate::write_llvm_ir;

#[test]
fn test_if_block() {
    let input = "
    let x = 1;
    const value = 2;
    
    if (value === 1) {
         x = 1;
    } 

   x = 30;
    ";

    let output = write_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(input, output);
}

#[test]
fn test_if_else_block() {
    let input = "
    let x = 1;
    const value = 2;
    
    if (value === 1) {
         x = 1;
    } else {
        x = 10
    }
    
   x = 30;
    ";

    let output = write_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(input, output);
}

#[test]
fn test_if_else_if_block() {
    let input = "
    let x = 1;
    const value = 2;
    
    if (value === 1) {
         x = 1;
    } else if (value === 2) {
        x = 2
    } else if (value === 3) {
        x = 3
    }

   x = 30;
    ";

    let output = write_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(input, output);
}

#[test]
fn test_if_else_if_else_block() {
    let input = "
    let x = 1;
    const value = 2;
    
    if (value === 1) {
         x = 1;
    } else if (value === 2) {
        x = 2
    } else if (value === 3) {
        x = 3
    } else {
        x = 10
    }
    
   x = 30;
    ";

    let output = write_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(output);
}

#[test]
fn test_var_dec_in_if_block() {
    let input = "
    const x = 1;
    
    if (x === 1) {
        const y = 2;
    } else {
        const y = 3;
    }
";

    let output = write_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(output);
}
