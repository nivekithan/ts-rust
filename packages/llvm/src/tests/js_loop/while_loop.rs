use lexer::convert_to_token;
use parser::convert_to_ast;

use crate::compile_to_llvm_ir;

#[test]
fn test_while_loop() {
    let input = "
    let x = 10;
    
    while (x !== 0) {
        x -= 1;    
    }";

    let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(output);
}

#[test]
fn test_while_loop_1() {
    let input = "
    let x = 10;
    
    while (x !== 0) {
       const y = x;
        x -= 1;    
    }";

    let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(output);
}

#[test]
fn test_while_loop_with_break() {
    let input = "
    const x = 1;
    
    while (x !== 0) {
        break;
    }";

    let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(input, output);
}

#[test]
fn test_while_loop_with_continue() {
    let input = "
    const x = 1;
    
    while (x !== 0) {
        continue;
    }";

    let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(input, output);
}

#[test]
fn test_function_call_inside_while_loop() {
    let input = "
    function foo() : void {
        const x = 1;
        return;
    }

    let x = 0;

    while (x !== 10) {
        foo();
        x += 1;
    }";

    let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input)));

    println!("{}", output);

    insta::assert_snapshot!(input, output);
}
