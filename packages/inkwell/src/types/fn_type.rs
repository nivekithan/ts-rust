use llvm_sys::prelude::LLVMTypeRef;

use super::{traits::AsTypeRef, Type};

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
}

impl<'a> AsTypeRef for FunctionType<'a> {
    fn as_type_ref(&self) -> LLVMTypeRef {
        return self.fn_type.as_type_ref();
    }
}
