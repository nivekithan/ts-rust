use std::collections::HashMap;

use inkwell::context::Context;
use lexer::convert_to_token;
use parser::{parse_main, resolver::Resolver as ParserResolver};

use crate::{compile_parser_resolver_to_llvm_module, link_llvm_module_resolver};

#[test]
fn test_simple_import() {
    let main_file = "
    import { foo } from \"foo\";

    const y = foo(5)

    ";

    let foo_file = "

    export function foo(x : number) : number {
        return x + 5;
    };
    ";

    let mut dependent_files: HashMap<String, String> = HashMap::new();
    dependent_files.insert("foo".to_string(), foo_file.to_string());

    let mut parser_resolver = ParserResolver::from(dependent_files.clone());
    parse_main(convert_to_token(main_file), &mut parser_resolver);

    let context = Context::create();

    let llvm_resolver = compile_parser_resolver_to_llvm_module(parser_resolver, &context);
    let linked_module = link_llvm_module_resolver(llvm_resolver);

    let main_content = linked_module.get_string_representation().to_string();

    let input = format!(
        "
Main file:
{}

Foo File:
{}
",
        main_file, foo_file
    );

    insta::assert_snapshot!(input, main_content);
}

#[test]
fn test_complex_import() {
    let main_file = "
    import {foo} from \"foo\";
    import {boo} from \"boo\";

    const y = foo(5);
    const z = boo(y);
    ";

    let foo_file = "
    import {boo} from \"boo\";

    export function foo(x : number) : number {
        return boo(x) + 5;
    };
    ";

    let boo_file = "
    export function boo(x : number) : number {
        return 5*x;
    };
    ";

    let mut dependent_files: HashMap<String, String> = HashMap::new();
    dependent_files.insert("foo".to_string(), foo_file.to_string());
    dependent_files.insert("boo".to_string(), boo_file.to_string());
    let mut parser_resolver = ParserResolver::from(dependent_files.clone());
    parse_main(convert_to_token(main_file), &mut parser_resolver);

    let context = Context::create();

    let llvm_resolver = compile_parser_resolver_to_llvm_module(parser_resolver, &context);
    let linked_module = link_llvm_module_resolver(llvm_resolver);

    let main_content = linked_module.get_string_representation().to_string();

    //     let input = format!(
    // "
    // Main file:
    // {}

    // Foo File:
    // {}

    // Boo File:
    // {}
    // ", main_file, foo_file, boo_file);

    insta::assert_snapshot!(main_content);
}
