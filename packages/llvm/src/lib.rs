use ast::Ast;
use codegen::Codegen;
use inkwell::context::Context;

mod codegen;

pub fn write_llvm_ir(content: Vec<Ast>) {
    let mut codgen = Codegen::new(&content);

    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();

    let main_fn_type = context.void_type().fn_type(&[], false);
    let main_fn = module.add_function("main", main_fn_type, None);

    let entry = context.append_basic_block(main_fn, "entry");
    builder.position_at_end(entry);

    codgen.consume(&context, &builder);

    let content = module.print_to_string();
    println!("{}", content.to_string());
}

#[cfg(test)]
mod test {
    use lexer::convert_to_token;
    use parser::convert_to_ast;

    use crate::write_llvm_ir;

    #[test]
    fn testing_add() {
        let input = "
        const  x =  1;
        const s = 23 + 323 - 23324 * 23 / 333";

        write_llvm_ir(convert_to_ast(convert_to_token(input)));
    }
}
