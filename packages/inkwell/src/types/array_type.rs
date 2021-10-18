use llvm_sys::{core::LLVMGetArrayLength, prelude::LLVMTypeRef};

use super::{
    traits::{AsTypeRef, BasicTypeTrait},
    Type,
};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct ArrayType<'a> {
    array_type: Type<'a>,
}

impl<'a> ArrayType<'a> {
    pub(crate) unsafe fn new(array_type: LLVMTypeRef) -> Self {
        assert!(!array_type.is_null());

        return ArrayType {
            array_type: Type::new(array_type),
        };
    }

    pub fn get_length(&self) -> u32 {
        unsafe {
            let length = LLVMGetArrayLength(self.as_type_ref());
            return length;
        }
    }
}

impl<'a> AsTypeRef for ArrayType<'a> {
    fn as_type_ref(&self) -> LLVMTypeRef {
        return self.array_type.as_type_ref();
    }
}

impl<'a> BasicTypeTrait<'a> for ArrayType<'a> {}
