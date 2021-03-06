use lexer::convert_to_token;
use parser::convert_to_ast;

use crate::compile_to_llvm_ir;

#[test]
fn do_while_loop() {
    let input = "
    let x = 10;
    
    do {
        x -= 1;
    } while (x !== 0)
    ";

    let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(output);
}

#[test]
fn do_while_loop_with_break() {
    let input = "
    const x = 0;
    do {
        break
    } while (x !== 1)";

    let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(input, output);
}

#[test]
fn do_while_loop_with_continue() {
    let input = "
    const x = 0;
    do {
        continue
    } while (x !== 1)";

    let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(input, output);
}
