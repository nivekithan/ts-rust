use std::collections::HashMap;

use lexer::convert_to_token;
use parser::{parse_main, resolver::Resolver as ParserResolver};

use crate::compile_parser_resolver_to_llvm_ir;

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

    let llvm_resolver = compile_parser_resolver_to_llvm_ir(parser_resolver);

    let main_content = &llvm_resolver.main;

    if let Some(main_content) = main_content {
        insta::assert_snapshot!(main_file, main_content);
    } else {
        insta::assert_snapshot!(main_file, "");
    }

    let keys: Vec<String> = {
        let mut keys: Vec<String> = llvm_resolver
            .dependencies
            .iter()
            .map(|(name, _)| {
                return name.to_string();
            })
            .collect();

        keys.sort_unstable();

        keys
    };

    for file_name in keys {
        let content = llvm_resolver.dependencies.get(&file_name).unwrap();
        let dependent_source_code = dependent_files.get(&file_name).unwrap();
        insta::assert_snapshot!(dependent_source_code.as_str(), content);
    }
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

    let llvm_resolver = compile_parser_resolver_to_llvm_ir(parser_resolver);

    let main_content = &llvm_resolver.main;

    if let Some(main_content) = main_content {
        insta::assert_snapshot!(main_file, main_content);
    } else {
        insta::assert_snapshot!(main_file, "");
    }

    let keys: Vec<String> = {
        let mut keys: Vec<String> = llvm_resolver
            .dependencies
            .iter()
            .map(|(name, _)| {
                return name.to_string();
            })
            .collect();

        keys.sort_unstable();

        keys
    };

    for file_name in keys {
        let content = llvm_resolver.dependencies.get(&file_name).unwrap();
        let dependent_source_code = dependent_files.get(&file_name).unwrap();
        insta::assert_snapshot!(dependent_source_code.as_str(), content);
    }
}
