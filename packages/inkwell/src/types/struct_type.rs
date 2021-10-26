use llvm_sys::prelude::LLVMTypeRef;

use super::{
    traits::{AsTypeRef, BasicTypeTrait},
    Type,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct StructType<'a> {
    struct_type: Type<'a>,
}

impl<'a> StructType<'a> {
    pub(crate) unsafe fn new(struct_type: LLVMTypeRef) -> Self {
        assert!(!struct_type.is_null());

        return StructType {
            struct_type: Type::new(struct_type),
        };
    }
}

impl<'a> AsTypeRef for StructType<'a> {
    fn as_type_ref(&self) -> LLVMTypeRef {
        return self.struct_type.as_type_ref();
    }
}

impl<'a> BasicTypeTrait<'a> for StructType<'a> {}
