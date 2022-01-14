use std::collections::HashMap;

use ast::Ast;
use codegen::Codegen;
use inkwell::{context::Context, enums::Linkage, module::Module};
use parser::resolver::Resolver as ParserResolver;
use resolver::Resolver;

mod build_assignment;
mod build_expression;
mod codegen;
mod enums;
mod gen_ast;
mod llvm_utils;
mod resolver;
mod symbol_table;
#[cfg(test)]
mod tests;
mod utils;

pub fn compile_to_llvm_ir(content: Vec<Ast>) -> String {
    let context = Context::create();
    let module = compile_to_llvm_module(content, &context, "main", true);
    return module.get_string_representation().to_string();
}

pub fn compile_to_llvm_module<'a>(
    content: Vec<Ast>,
    context: &'a Context,
    module_name: &str,
    is_main_file: bool,
) -> Module<'a> {
    let mut codegen = Codegen::new(&content);
    let module = context.create_module(module_name);
    let builder = context.create_builder();
    let main_fn_type = context.void_type().fn_type(&[], false);
    let linkage_type = {
        if is_main_file {
            None
        } else {
            Some(Linkage::Private)
        }
    };
    let mut main_fn = module.add_function("main", main_fn_type, linkage_type);

    let entry = context.append_basic_block(&main_fn, "entry");
    builder.position_at_end(&entry);

    codegen.consume(&context, &builder, &module, &mut main_fn);
    builder.build_return(None);

    if cfg!(test) {
        if let Err(err_str) = module.verify() {
            println!("{}", err_str.to_string());
        }
    }
    return module;
}

pub fn compile_parser_resolver_to_llvm_module<'a>(
    parser_resolver: ParserResolver,
    context: &'a Context,
) -> Resolver<Module<'a>> {
    let main_data = parser_resolver.get_main().clone().unwrap();

    let main_module = compile_to_llvm_module(main_data.ast, &context, "main", true);

    let mut dependencies: HashMap<String, Module<'a>> = HashMap::new();

    let parser_dependencies = parser_resolver.get_dependencies();
    parser_dependencies.iter().for_each(|file_name| {
        let data = parser_resolver.get_data(file_name);

        let dependent_content =
            compile_to_llvm_module(data.ast.clone(), &context, file_name, false);

        dependencies.insert(file_name.to_string(), dependent_content);
    });

    return Resolver {
        dependencies,
        main: Some(main_module),
    };
}

pub fn link_llvm_module_resolver<'a>(resolver: Resolver<Module<'a>>) -> Module<'a> {
    let main = resolver.main.unwrap();

    for (_, module) in resolver.dependencies {
        main.link_module(module).unwrap();
    }

    return main;
}

#[cfg(test)]
mod test_1 {
    use std::convert::TryInto;

    use either::Either;
    use inkwell::{
        context::Context, enums::InlineAsmSyntax, types::traits::BasicTypeTrait,
        values::traits::BasicValueTrait,
    };

    #[test]

    fn test() {
        let context = Context::create();
        let module = context.create_module("main");
        let builder = context.create_builder();

        let main_fn_type = context.void_type().fn_type(&[], false);
        let mut main_fn = module.add_function("main", main_fn_type, None);

        let entry = context.append_basic_block(&main_fn, "entry");

        builder.position_at_end(&entry);

        // [BEGIN]--------- Adding String "Hello world! in stack" -----------
        let value = "Hello World!";
        let length = value.len();

        let size_of_string = value.len() as u32;
        let string_array_type = context.i8_type().array_type(size_of_string);

        let base_pointer = builder.build_alloca(string_array_type, "x_");

        for (i, c) in value.chars().enumerate() {
            let indices = vec![
                context.i64_type().const_int(0, true),
                context.i64_type().const_int(i.try_into().unwrap(), true),
            ];

            let index_pointer = builder.build_gep_2(
                string_array_type,
                &base_pointer,
                &indices,
                &main_fn.get_unique_reg_name(),
            );

            let char_value = context.i8_type().const_int(c as u64, false);
            builder.build_store(index_pointer, char_value);
        }

        // [END]--------- Adding String "Hello world! in stack" -----------

        let asm_type = context.void_type().fn_type(
            &[
                context.i64_type().as_basic_type_enum(),
                context.i64_type().as_basic_type_enum(),
                base_pointer.get_type().as_basic_type_enum(),
                context.i64_type().as_basic_type_enum(),
            ],
            false,
        );

        let inline_asm = asm_type.create_inline_asm(
            "syscall",
            "{rax},{rdi},{rsi},{rdx}",
            true,
            false,
            InlineAsmSyntax::Att,
        );

        builder.build_call2(
            Either::Right(&inline_asm),
            &[
                context.i64_type().const_int(1, true).as_basic_value_enum(),
                context.i64_type().const_int(1, true).as_basic_value_enum(),
                base_pointer.as_basic_value_enum(),
                context
                    .i64_type()
                    .const_int(length as u64, true)
                    .as_basic_value_enum(),
            ],
            "as",
        );

        builder.build_return(None);

        let output = module.get_string_representation().to_string();
        insta::assert_snapshot!("Testing: print(\"Hello World!\")", output);
    }
}
