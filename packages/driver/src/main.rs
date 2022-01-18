use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use inkwell::context::Context;
use lexer::convert_to_token;
use llvm::{
    compile_parser_resolver_to_llvm_module, link_llvm_module_resolver, write_assembly_file_to_path,
};
use parser::{parse_main, parser_resolver::ParserResolver};
use path_absolutize::Absolutize;

fn main() {
    let main_file_path = get_main_file_path();

    let main_file_code = get_file_content(&main_file_path);
    let mut parser_resolver = ParserResolver::from(
        HashMap::new(),
        Box::new(|file_name| {
            return Ok(get_file_content(&get_absolute_file_path(file_name)));
        }),
    );

    parse_main(
        convert_to_token(&main_file_code),
        &mut parser_resolver,
        main_file_path.to_str().unwrap(),
    );
    let context = Context::create();
    let llvm_resolver = compile_parser_resolver_to_llvm_module(parser_resolver, &context);
    let final_module = link_llvm_module_resolver(llvm_resolver);
    let assembly_path = get_assembly_file_path();

    write_assembly_file_to_path(&final_module, &assembly_path);
    compile_assembly_to_exec();
}

fn get_assembly_file_path() -> PathBuf {
    let file_name = "./output.s";
    return get_absolute_file_path(file_name);
}

fn get_main_file_path() -> PathBuf {
    let name = std::env::args().nth(1).unwrap();
    return get_absolute_file_path(&name);
}

fn get_absolute_file_path(name: &str) -> PathBuf {
    let mut cwd = std::env::current_dir().unwrap();
    cwd.push(Path::new(name));
    return Path::new(cwd.absolutize().unwrap().to_str().unwrap()).to_path_buf();
}

fn get_file_content(file_path: &PathBuf) -> String {
    let content = fs::read_to_string(file_path).unwrap();
    return content;
}

fn compile_assembly_to_exec() {
    let assembly_file_path = get_assembly_file_path();
    Command::new("gcc")
        .args([assembly_file_path.to_str().unwrap(), "-o", "output"])
        .status()
        .unwrap();
}
