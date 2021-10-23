use ast::Ast;
use codegen::Codegen;
use inkwell::context::Context;

mod build_assignment;
mod build_expression;
mod codegen;
mod enums;
mod gen_ast;

#[cfg(test)]
mod tests;

pub fn write_llvm_ir(content: Vec<Ast>) -> String {
    let mut codgen = Codegen::new(&content);

    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();

    let main_fn_type = context.void_type().fn_type(&[], false);
    let mut main_fn = module.add_function("main", main_fn_type, None);

    let entry = context.append_basic_block(&main_fn, "entry");
    builder.position_at_end(&entry);

    codgen.consume(&context, &builder, &mut main_fn);

    let content = module.print_to_string().to_string();
    return content;
}

#[cfg(test)]
mod test_1 {
    use lexer::convert_to_token;
    use parser::convert_to_ast;

    use crate::write_llvm_ir;

    #[test]
    fn test_some() {
        let input = "
        const x = [1,1]
        const z = 1;
        const y = x[z]";

        let output = write_llvm_ir(convert_to_ast(convert_to_token(input)));

        println!("{}", output);
    }
}
