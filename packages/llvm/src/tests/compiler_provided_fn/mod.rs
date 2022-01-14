use lexer::convert_to_token;
use parser::convert_to_ast;

use crate::write_llvm_ir;




#[test]
fn test_syscall_1() {
    let input = "
    import {syscallPrint} from \"compilerInternal\";

    const name = \"Nivekithan\";
    syscallPrint(1, name, 10);
    ";

    let output = write_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(input, output);
}