use std::marker::PhantomData;

use llvm_sys::{
    core::{LLVMPrintValueToString, LLVMTypeOf},
    prelude::{LLVMTypeRef, LLVMValueRef},
};

use crate::utils::llvm_string::LLVMString;

use self::traits::AsValueRef;

pub mod enums;
pub mod float_value;
pub mod fn_value;
pub mod instruction_value;
pub mod int_value;
pub mod ptr_value;
pub mod traits;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]

pub(crate) struct Value<'a> {
    pub(crate) value: LLVMValueRef,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Value<'a> {
    pub(crate) unsafe fn new(value: LLVMValueRef) -> Self {
        assert!(!value.is_null());

        return Value {
            value,
            _marker: PhantomData,
        };
    }

    pub fn print_value(&self) {
        unsafe {
            let value = LLVMPrintValueToString(self.value);
            let llvm_string = LLVMString::new(value).to_string();
            println!("{}", llvm_string);
        }
    }

    pub(crate) fn get_type(&self) -> LLVMTypeRef {
        unsafe {
            return LLVMTypeOf(self.as_value_ref());
        }
    }
}

impl<'a> AsValueRef for Value<'a> {
    fn as_value_ref(&self) -> LLVMValueRef {
        return self.value;
    }
}
