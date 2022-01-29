mod cmd_import_resolver;
mod file_unique_id;
mod utils;
use cmd_import_resolver::{compile_assembly_to_exec, CommandLineResolver};
use inkwell::context::Context;
use llvm::write_assembly_file_to_path;

fn main() {
    let context = Context::create();
    let command_line_resolver = CommandLineResolver::new();
    let assembly_path = command_line_resolver.get_assembly_file_path();
    let final_module = command_line_resolver.compile(&context);

    write_assembly_file_to_path(&final_module, &assembly_path);
    compile_assembly_to_exec(&assembly_path);
}
