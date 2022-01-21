mod cmd_import_resolver;
mod utils;
use cmd_import_resolver::CommandLineResolver;
use inkwell::context::Context;
use llvm::write_assembly_file_to_path;

fn main() {
    let context = Context::create();
    let mut command_line_resolver = CommandLineResolver::new();
    let final_module = command_line_resolver.compile(&context);
    let assembly_path = command_line_resolver.get_assembly_file_path();

    write_assembly_file_to_path(&final_module, &assembly_path);
    command_line_resolver.compile_assembly_to_exec();
}
