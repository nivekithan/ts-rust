use either::Either;
use inkwell::{
    context::Context,
    enums::InlineAsmSyntax,
    module::Module,
    types::{enums::AddressSpace, traits::BasicTypeTrait},
    values::{enums::BasicValueEnum, traits::BasicValueTrait},
};

pub fn get_compiler_provided_module<'a>(context: &'a Context) -> Module<'a> {
    let module = context.create_module("compilerInternal");
    build_syscall_print(context, &module);

    return module;
}

fn build_syscall_print<'a>(context: &'a Context, module: &'a Module<'a>) {
    let fn_type = context.void_type().fn_type(
        &[
            context.f64_type().as_basic_type_enum(),
            context
                .i8_type()
                .ptr_type(AddressSpace::Generic)
                .as_basic_type_enum(),
            context.f64_type().as_basic_type_enum(),
        ],
        false,
    );

    let name = "|fn:1|syscallPrint|_|";
    let fn_value = module.add_function(name, fn_type, None);
    let entry = context.append_basic_block(&fn_value, "entry");

    let builder = context.create_builder();
    builder.position_at_end(&entry);

    let param_1 = fn_value.get_nth_param(0).unwrap();
    let param_2 = fn_value.get_nth_param(1).unwrap();
    let param_3 = fn_value.get_nth_param(2).unwrap();
    if let BasicValueEnum::FloatValue(param_1) = param_1 {
        if let BasicValueEnum::PointerValue(param_2) = param_2 {
            if let BasicValueEnum::FloatValue(param_3) = param_3 {
                let rdi_int =
                    builder.build_float_to_signed_int(param_1, context.i64_type(), "rdi_int");
                let size_int =
                    builder.build_float_to_signed_int(param_3, context.i64_type(), "size_int");

                let asm_type = context.void_type().fn_type(
                    &[
                        context.i64_type().as_basic_type_enum(),
                        context.i64_type().as_basic_type_enum(),
                        param_2.get_type().as_basic_type_enum(),
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
                        rdi_int.as_basic_value_enum(),
                        param_2.as_basic_value_enum(),
                        size_int.as_basic_value_enum(),
                    ],
                    "",
                );
            }
        }
    }

    builder.build_return(None);
}
