use std::path::Path;

use inkwell::context::Context;
use lexer::convert_to_token;
use parser::convert_to_ast;

use crate::{compile_to_llvm_module, write_assembly_file_to_path};

mod compiler_provided_fn;
mod control_flow;
mod functions;
mod js_loop;
mod modules;
mod naked_expression;
mod variable_declaration;

#[test]
fn test_writing_assembly() {
    let input = "
    const s = \"124\"";

    let context = Context::create();
    let module = compile_to_llvm_module(
        convert_to_ast(convert_to_token(input)),
        &context,
        "main",
        true,
    );
    let path = Path::new("../../../../temp/test2.s");

    write_assembly_file_to_path(&module, path);
}
