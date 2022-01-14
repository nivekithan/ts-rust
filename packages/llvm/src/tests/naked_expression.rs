use lexer::convert_to_token;
use parser::convert_to_ast;

use crate::compile_to_llvm_ir;

#[test]
fn test_naked_float_expression() {
    let input = "
    5 + 5";

    let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(input, output);
}

#[test]
fn test_naked_ident_expression() {
    let input = "
    const x = 10;
    x;";

    let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(input, output);
}

#[test]
fn test_naked_function_expression() {
    let input = "
    function foo(x : number) : number {
        return x + 1;
    }
    
    foo(1);
    ";

    let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(input, output);
}
