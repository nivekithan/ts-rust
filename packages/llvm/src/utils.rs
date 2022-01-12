use ast::data_type::DataType;
use inkwell::{
    context::Context,
    module::Module,
    types::{enums::BasicTypeEnum, fn_type::FunctionType},
    values::fn_value::FunctionValue,
};

use crate::llvm_utils::LLVMUtils;

/*
 *
 * Both fn `get_personality_fn` and `create_personality_fn` are just an  temporary hacky solution
 * so that I can make builder.build_invoke_2 work without llvm throwing error
 * */
#[allow(dead_code)]
pub(crate) fn get_personality_fn<'a>(module: &'a Module<'a>) -> FunctionValue<'a> {
    let personality_fn_name = "__typescript__personality";
    let fn_value = module.get_fn_value(personality_fn_name);
    return fn_value;
}

#[allow(dead_code)]
pub(crate) fn create_personality_fn<'a>(module: &Module<'a>) {
    let personality_fn_name = "__typescript__personality";

    let context = module.get_context();
    let fn_type = context.void_type().fn_type(&[], false);
    let fn_value = module.add_function(personality_fn_name, fn_type, None);

    let builder = context.create_builder();

    let entry_bb = context.append_basic_block(&fn_value, "entry");
    builder.position_at_end(&entry_bb);

    builder.build_return(None);
}

pub(crate) fn convert_function_data_type_to_llvm_function_type<'a>(
    data_type: &DataType,
    context: &'a Context,
) -> FunctionType<'a> {
    if let DataType::FunctionType {
        arguments,
        return_type,
    } = data_type
    {
        let return_type = return_type.force_to_basic_type(context);
        let arguments: Vec<BasicTypeEnum> = arguments
            .iter()
            .map(|data_type| return data_type.force_to_basic_type(context))
            .collect();
        let fn_type = return_type.fn_type(&arguments, false);
        return fn_type;
    } else {
        panic!(
            "Expected data type to be DataType::FunctionType but got {:?}",
            data_type
        );
    }
}
