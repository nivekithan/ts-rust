use llvm_sys::{
    core::{LLVMGetArrayLength, LLVMGetElementType},
    prelude::LLVMTypeRef,
};

use super::{
    enums::BasicTypeEnum,
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

    pub fn get_element_type(&self) -> BasicTypeEnum {
        unsafe {
            let element_type = LLVMGetElementType(self.as_type_ref());
            let basic_type = BasicTypeEnum::new(element_type);
            return basic_type;
        }
    }
}

impl<'a> AsTypeRef for ArrayType<'a> {
    fn as_type_ref(&self) -> LLVMTypeRef {
        return self.array_type.as_type_ref();
    }
}

impl<'a> BasicTypeTrait<'a> for ArrayType<'a> {}
