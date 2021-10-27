use lexer::convert_to_token;
use parser::convert_to_ast;

use crate::write_llvm_ir;


#[test]
fn test_simple_fn_declaration() {
    let input = "
    function foo(x : number, a : number, c : boolean) : boolean {
        

        return !false;
    }";

    
    let output = write_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(input, output);
}