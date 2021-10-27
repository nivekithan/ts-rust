use ast::data_type::DataType;
use inkwell::{
    context::Context,
    types::{enums::BasicTypeEnum, traits::BasicTypeTrait},
};

pub(crate) trait LLVMUtils<'a> {
    fn to_basic_type(&self, context: &'a Context) -> BasicTypeEnum<'a>;
}

impl<'a> LLVMUtils<'a> for DataType {
    fn to_basic_type(&self, context: &'a Context) -> BasicTypeEnum<'a> {
        match self {
            DataType::Float => context.f64_type().as_basic_type_enum(),
            DataType::Boolean => context.i1_type().as_basic_type_enum(),

            _ => panic!("Cannot convert data_type {:?} to BasicTypeEnum", self),
        }
    }
}
