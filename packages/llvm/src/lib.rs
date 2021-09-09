use ast::Ast;
use codegen::Codegen;
use inkwell::context::Context;

mod codegen;

#[cfg(test)]
mod tests;

pub fn write_llvm_ir(content: Vec<Ast>) -> String {
    let mut codgen = Codegen::new(&content);

    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();

    let main_fn_type = context.void_type().fn_type(&[], false);
    let main_fn = module.add_function("main", main_fn_type, None);

    let entry = context.append_basic_block(main_fn, "entry");
    builder.position_at_end(entry);

    codgen.consume(&context, &builder);

    let content = module.print_to_string().to_string();
    return content;
}
