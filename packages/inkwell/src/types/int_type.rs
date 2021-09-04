use llvm_sys::{core::LLVMConstInt, prelude::LLVMTypeRef};

use crate::values::int_value::IntValue;

use super::{
    traits::{AsTypeRef, BasicTypeTrait, IntMathTypeTrait},
    Type,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IntType<'a> {
    int_type: Type<'a>,
}

impl<'a> IntType<'a> {
    pub(crate) unsafe fn new(int_type: LLVMTypeRef) -> Self {
        assert!(!int_type.is_null());

        return IntType {
            int_type: Type::new(int_type),
        };
    }

    pub fn const_int(self, value: u64, sign_extended: bool) -> IntValue<'a> {
        unsafe {
            return IntValue::new(LLVMConstInt(
                self.as_type_ref(),
                value,
                sign_extended as i32,
            ));
        }
    }
}

impl<'a> AsTypeRef for IntType<'a> {
    fn as_type_ref(&self) -> LLVMTypeRef {
        return self.int_type.as_type_ref();
    }
}

impl<'a> BasicTypeTrait<'a> for IntType<'a> {}

impl<'a> IntMathTypeTrait<'a> for IntType<'a> {
    type ValueType = IntValue<'a>;
}
