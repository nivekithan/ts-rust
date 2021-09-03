use llvm_sys::prelude::LLVMTypeRef;

use super::{
    traits::{AsTypeRef, BasicTypeTrait},
    Type,
};

pub struct PointerType<'a> {
    ptr_type: Type<'a>,
}

impl<'a> PointerType<'a> {
    pub(crate) unsafe fn new(ptr_type: LLVMTypeRef) -> Self {
        assert!(!ptr_type.is_null());

        return PointerType {
            ptr_type: Type::new(ptr_type),
        };
    }
}

impl<'a> AsTypeRef for PointerType<'a> {
    fn as_type_ref(&self) -> LLVMTypeRef {
        return self.ptr_type.as_type_ref();
    }
}

impl<'a> BasicTypeTrait<'a> for PointerType<'a> {}
