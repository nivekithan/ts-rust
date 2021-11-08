use llvm_sys::{core::LLVMGetReturnType, prelude::LLVMTypeRef};

use super::{enums::BasicTypeEnum, traits::AsTypeRef, utils::print_type_ref, Type};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct FunctionType<'a> {
    fn_type: Type<'a>,
}

impl<'a> FunctionType<'a> {
    pub(crate) unsafe fn new(fn_type: LLVMTypeRef) -> Self {
        assert!(!fn_type.is_null());

        return FunctionType {
            fn_type: Type::new(fn_type),
        };
    }

    pub fn get_return_type(&self) -> BasicTypeEnum<'a> {
        unsafe {
            let return_type = LLVMGetReturnType(self.as_type_ref());
            print_type_ref(return_type);
            return BasicTypeEnum::new(return_type);
        }
    }
}

impl<'a> AsTypeRef for FunctionType<'a> {
    fn as_type_ref(&self) -> LLVMTypeRef {
        return self.fn_type.as_type_ref();
    }
}
