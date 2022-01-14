use lexer::convert_to_token;
use parser::convert_to_ast;

use crate::compile_to_llvm_ir;

#[test]
fn test_simple_fn_declaration() {
    let input = "
    function foo(x : number, a : number, c : boolean) : boolean {
        

        return !false;
    }";

    let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(input, output);
}

#[test]
fn test_using_parameters() {
    let input = "
    function foo(x : number, a : number, c : boolean) : number {
        
        return x + a;
    }";

    let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(input, output);
}

#[test]
fn test_calling_a_function() {
    let input = "
    function foo(x : number) : number {
        return x + 1;
    }
    
    let y = foo(5);";

    let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(input, output);
}

#[test]
fn test_calling_a_function_with_void_return_type() {
    let input = "
    function foo(x : number) : void {
        return;
    }
    
    foo(5);";

    let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(input, output);
}

#[test]
#[should_panic]
fn test_assigning_a_variable_to_function_that_returns_void() {
    let input = "
    function foo(x : number) : void {
        return;
    }
    
    const x = foo(5);";

    compile_to_llvm_ir(convert_to_ast(convert_to_token(input)));
}

#[test]
fn test_assigning_a_function_to_variable() {
    let input = "
    function foo(x : number) : void {
        return;
    }
    
    const x = foo;
    x()
    ";

    let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(input, output);
}

#[test]
fn test_passing_a_string_to_function() {
    let input = "
    function foo(x : string) : void {
        return;
    };
    
    foo(\"1233\");
    ";
    let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(input, output);
}

#[test]
fn test_passing_a_callback_function() {
    let input = "
    function foo(x : number, y : (s : number) => number, ) : void {
        y(x);
        return;
    };

    function bar(y : number) : number {
        return y + y;
    }

    foo(5, bar);
    ";

    let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(input, output);
}

#[test]
fn test_passing_object_to_function() {
    let input = "
    function foo(x : {a : number}) : number {
        return x.a;
    };

    const y = foo({a : 5});
    
    ";

    let output = compile_to_llvm_ir(convert_to_ast(convert_to_token(input)));

    insta::assert_snapshot!(input, output);
}
