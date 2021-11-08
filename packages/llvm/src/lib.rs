use ast::Ast;
use codegen::Codegen;
use inkwell::context::Context;
use utils::create_personality_fn;

mod build_assignment;
mod build_expression;
mod codegen;
mod enums;
mod gen_ast;
mod llvm_utils;
mod utils;

#[cfg(test)]
mod tests;

pub fn write_llvm_ir(content: Vec<Ast>) -> String {
    let mut codgen = Codegen::new(&content);

    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();
    let main_fn_type = context.void_type().fn_type(&[], false);
    let mut main_fn = module.add_function("main", main_fn_type, None);

    create_personality_fn(&module);

    let entry = context.append_basic_block(&main_fn, "entry");
    builder.position_at_end(&entry);

    codgen.consume(&context, &builder, &module, &mut main_fn);

    let content = module.get_string_representation().to_string();
    return content;
}

#[cfg(test)]
mod test_1 {
  
    // use inkwell::{context::Context, types::traits::BasicTypeTrait};
    use lexer::convert_to_token;
    use parser::convert_to_ast;

    use crate::write_llvm_ir;

    #[test]
    fn test_some() {
        let input = "
        function foo(x : number) : number {
            const y = 2;
            const c = 1;
            const z = y * c + 42 ;
        }
        let x = foo(5);
        ";

        let output = write_llvm_ir(convert_to_ast(convert_to_token(input)));

        println!("{}", output);
    }

    // #[test]
    // fn test_foo() {
    //     let context = Context::create();
    //     let module = context.create_module("main");

    //     let main_fn_type = context.void_type().fn_type(&[], false);
    //     let mut main_fn = module.add_function("main", main_fn_type, None);

    //     let second_fn_type = context.void_type().fn_type(&[], false);
    //     let mut second_fn = module.add_function("foo_", second_fn_type, None);

    //     let entry = context.append_basic_block(&main_fn, "entry");
    //     builder.position_at_end(&entry);

    //     builder.build_return(Some(&context.f64_type().const_float(12.0)));

    //     println!("{}", module.print_to_string().to_string());
    // }

    // #[test]
    // fn test_boo() {
    //     let context = Context::create();
    //     let module = context.create_module("main");

    //     let main_fn_type = context.void_type().fn_type(&[], false);
    //     let main_fn = module.add_function("main", main_fn_type, None);

    //     let builder = context.create_builder();

    //     let entry = context.append_basic_block(&main_fn, "entry");
    //     builder.position_at_end(&entry);

    //     builder.build_return(None);

    //     let main_fn_type = main_fn.get_type();
    //     unsafe {
    //         print_type_ref(main_fn_type.as_type_ref());
    //     }

    //     let return_type = main_fn_type.get_return_type();

    //     // println!("Type is {:?}",return_type);
    // }
}
