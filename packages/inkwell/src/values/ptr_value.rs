use llvm_sys::prelude::LLVMValueRef;

use super::{
    traits::{AsValueRef, BasicValueTrait},
    Value,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct PointerValue<'a> {
    ptr_value: Value<'a>,
}

impl<'a> PointerValue<'a> {
    pub(crate) unsafe fn new(value: LLVMValueRef) -> Self {
        assert!(!value.is_null());

        return PointerValue {
            ptr_value: Value::new(value),
        };
    }
}

impl<'a> AsValueRef for PointerValue<'a> {
    fn as_value_ref(&self) -> LLVMValueRef {
        return self.ptr_value.as_value_ref();
    }
}

impl<'a> BasicValueTrait<'a> for PointerValue<'a> {}
