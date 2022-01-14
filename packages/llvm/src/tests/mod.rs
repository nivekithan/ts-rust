use std::{collections::HashMap, path::PathBuf};

use inkwell::context::Context;
use lexer::convert_to_token;
use parser::{parse_main, resolver::Resolver as ParserResolver};

use crate::{
    compile_parser_resolver_to_llvm_module, link_llvm_module_resolver, write_assembly_file_to_path,
};

mod compiler_provided_fn;
mod control_flow;
mod functions;
mod js_loop;
mod modules;
mod naked_expression;
mod variable_declaration;

#[test]
fn test_writing_assembly() {
    let main_file = "
    import {syscallPrint} from \"compilerInternal\";
    const s = \"124\"
    syscallPrint(1, s, 3);
    ";

    let dependent_files: HashMap<String, String> = HashMap::new();

    let mut parser_resolver = ParserResolver::from(dependent_files.clone());
    parse_main(convert_to_token(main_file), &mut parser_resolver);

    let context = Context::create();
    let llvm_resolver = compile_parser_resolver_to_llvm_module(parser_resolver, &context);
    let final_module = link_llvm_module_resolver(llvm_resolver);
    let mut cur_path = std::env::current_dir().unwrap();
    cur_path.push(PathBuf::from("../../temp/test2.s"));
    let path = cur_path.canonicalize().unwrap();

    write_assembly_file_to_path(&final_module, &path);
}
