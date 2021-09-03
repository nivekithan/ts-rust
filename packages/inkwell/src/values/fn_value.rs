use llvm_sys::{core::LLVMIsAFunction, prelude::LLVMValueRef};

use super::{traits::AsValueRef, Value};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]

pub struct FunctionValue<'a> {
    pub(crate) fn_value: Value<'a>,
}

impl<'a> FunctionValue<'a> {
    pub(crate) unsafe fn new(value: LLVMValueRef) -> Self {
        assert!(!value.is_null());

        assert!(!LLVMIsAFunction(value).is_null());

        return FunctionValue {
            fn_value: Value::new(value),
        };
    }
}

impl<'a> AsValueRef for FunctionValue<'a> {
    fn as_value_ref(&self) -> LLVMValueRef {
        return self.fn_value.as_value_ref();
    }
}
