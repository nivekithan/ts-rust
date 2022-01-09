use ast::data_type::DataType;
use inkwell::{
    context::Context,
    types::{
        enums::{AddressSpace, BasicTypeEnum},
        traits::{AsTypeRef, BasicTypeTrait},
        utils::print_type_ref,
    },
};

pub(crate) trait LLVMUtils<'a> {
    fn to_basic_type(&self, context: &'a Context) -> BasicTypeEnum<'a>;
}

impl<'a> LLVMUtils<'a> for DataType {
    fn to_basic_type(&self, context: &'a Context) -> BasicTypeEnum<'a> {
        match self {
            DataType::Float => context.f64_type().as_basic_type_enum(),
            DataType::Boolean => context.i1_type().as_basic_type_enum(),
            DataType::Void => context.void_type().as_basic_type_enum(),
            DataType::FunctionType {
                return_type,
                arguments,
            } => {
                let return_type = return_type.to_basic_type(context);
                let arguments: Vec<BasicTypeEnum> = arguments
                    .iter()
                    .map(|data_type| {
                        return data_type.to_basic_type(context);
                    })
                    .collect();

                let fn_type = return_type.fn_type(&arguments, false);
                //TODO: Remove Debug
                unsafe {
                    println!("Hello ");
                    print_type_ref(fn_type.as_type_ref());
                }
                // ------

                let ptr_type = fn_type.ptr_type(AddressSpace::Global);

                //TODO: Remove Debug
                unsafe {
                    println!("Hello ---- ");
                    print_type_ref(ptr_type.as_type_ref());
                }
                // ------

                return ptr_type.as_basic_type_enum();
            }

            _ => panic!("Cannot convert data_type {:?} to BasicTypeEnum", self),
        }
    }
}
