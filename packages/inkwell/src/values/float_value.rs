use llvm_sys::prelude::LLVMValueRef;

use crate::types::float_type::FloatType;

use super::{
    traits::{AsValueRef, BasicValueTrait, FloatMathValueTrait},
    Value,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FloatValue<'a> {
    pub(crate) float_value: Value<'a>,
}

impl<'a> FloatValue<'a> {
    pub(crate) unsafe fn new(value: LLVMValueRef) -> Self {
        assert!(!value.is_null());

        return FloatValue {
            float_value: Value::new(value),
        };
    }
}

impl<'a> AsValueRef for FloatValue<'a> {
    fn as_value_ref(&self) -> LLVMValueRef {
        return self.float_value.as_value_ref();
    }
}

impl<'a> BasicValueTrait<'a> for FloatValue<'a> {}

impl<'a> FloatMathValueTrait<'a> for FloatValue<'a> {
    type BaseType = FloatType<'a>;

    fn new(value_ref: LLVMValueRef) -> Self {
        unsafe {
            return Self::new(value_ref);
        }
    }
}
