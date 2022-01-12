use std::collections::HashMap;

use lexer::convert_to_token;
use parser::{parse_main, resolver::Resolver as ParserResolver};

use crate::consume_parser_resolver;

#[test]
fn test_import() {
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

    let mut parser_resolver = ParserResolver::from(dependent_files);
    parse_main(convert_to_token(main_file), &mut parser_resolver);

    let llvm_resolver = consume_parser_resolver(parser_resolver);

    println!(
        "Main File : 
 {}",
        &llvm_resolver.main.unwrap()
    );

    llvm_resolver
        .dependencies
        .iter()
        .for_each(|(name, content)| {
            println!(
                "
{} :
{}",
                name, content
            )
        });
}
