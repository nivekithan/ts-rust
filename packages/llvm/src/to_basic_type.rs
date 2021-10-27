use ast::data_type::DataType;
use inkwell::{
    context::Context,
    types::{enums::BasicTypeEnum, traits::BasicTypeTrait},
};

pub(crate) trait ToBasicType<'a> {
    fn to_llvm_type(&self, context: &'a Context) -> BasicTypeEnum<'a>;
}

impl<'a> ToBasicType<'a> for DataType {
    fn to_llvm_type(&self, context: &'a Context) -> BasicTypeEnum<'a> {
        match self {
            DataType::Float => context.f64_type().as_basic_type_enum(),
            DataType::Boolean => context.i1_type().as_basic_type_enum(),

            _ => panic!("Cannot convert data_type {:?} to BasicTypeEnum", self),
        }
    }
}
