use llvm_sys::prelude::LLVMValueRef;

use crate::types::int_type::IntType;

use super::{
    traits::{AsValueRef, BasicValueTrait, IntMathValueTrait},
    Value,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct IntValue<'a> {
    int_value: Value<'a>,
}

impl<'a> IntValue<'a> {
    pub(crate) unsafe fn new(value: LLVMValueRef) -> Self {
        assert!(!value.is_null());

        return IntValue {
            int_value: Value::new(value),
        };
    }
}

impl<'a> AsValueRef for IntValue<'a> {
    fn as_value_ref(&self) -> LLVMValueRef {
        return self.int_value.as_value_ref();
    }
}

impl<'a> BasicValueTrait<'a> for IntValue<'a> {}

impl<'a> IntMathValueTrait<'a> for IntValue<'a> {
    type BaseType = IntType<'a>;

    fn new(value_ref: LLVMValueRef) -> Self {
        unsafe {
            return Self::new(value_ref);
        }
    }
}
