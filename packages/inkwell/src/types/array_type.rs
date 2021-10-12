use llvm_sys::prelude::LLVMTypeRef;

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
}

impl<'a> AsTypeRef for ArrayType<'a> {
    fn as_type_ref(&self) -> LLVMTypeRef {
        return self.array_type.as_type_ref();
    }
}

impl<'a> BasicTypeTrait<'a> for ArrayType<'a> {}
