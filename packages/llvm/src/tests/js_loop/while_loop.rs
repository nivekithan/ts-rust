use lexer::convert_to_token;
use parser::convert_to_ast;

use crate::write_llvm_ir;

#[test]
fn test_while_loop() {
    let input = "
    let x = 10;
    
    while (x !== 0) {
        x -= 1;    
    }";

    let output = write_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(output);
}
