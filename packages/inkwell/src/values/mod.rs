use std::marker::PhantomData;

use llvm_sys::prelude::LLVMValueRef;

use self::traits::AsValueRef;

pub mod enums;
pub mod float_value;
pub mod fn_value;
pub mod instruction_value;
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
}

impl<'a> AsValueRef for Value<'a> {
    fn as_value_ref(&self) -> LLVMValueRef {
        return self.value;
    }
}
