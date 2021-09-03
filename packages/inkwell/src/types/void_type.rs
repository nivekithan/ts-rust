use llvm_sys::prelude::LLVMTypeRef;

use super::{
    enums::BasicTypeEnum,
    fn_type::FunctionType,
    traits::{AsTypeRef, BasicTypeTrait},
    Type,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VoidType<'a> {
    void_type: Type<'a>,
}

impl<'a> VoidType<'a> {
    pub(crate) unsafe fn new(void_type: LLVMTypeRef) -> Self {
        assert!(!void_type.is_null());

        return VoidType {
            void_type: Type::new(void_type),
        };
    }

    pub fn fn_type(
        self,
        param_types: &[BasicTypeEnum<'a>],
        variadic_arg: bool,
    ) -> FunctionType<'a> {
        return self.void_type.fn_type(param_types, variadic_arg);
    }
}

impl<'a> AsTypeRef for VoidType<'a> {
    fn as_type_ref(&self) -> LLVMTypeRef {
        return self.void_type.as_type_ref();
    }
}

impl<'a> BasicTypeTrait<'a> for VoidType<'a> {}
