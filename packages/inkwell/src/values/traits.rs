use llvm_sys::prelude::LLVMValueRef;

use crate::types::traits::{FloatMathTypeTrait, IntMathTypeTrait};

use super::enums::BasicValueEnum;

pub trait AsValueRef {
    fn as_value_ref(&self) -> LLVMValueRef;
}

pub trait BasicValueTrait<'a>: AsValueRef {
    fn as_basic_value_enum(&self) -> BasicValueEnum<'a> {
        unsafe { return BasicValueEnum::new(self.as_value_ref()) }
    }
}

// Types which can be used in FloatMathOperation implements it
pub trait FloatMathValueTrait<'a>: BasicValueTrait<'a> {
    type BaseType: FloatMathTypeTrait<'a>;

    fn new(value_ref: LLVMValueRef) -> Self;
}

pub trait IntMathValueTrait<'a>: BasicValueTrait<'a> {
    type BaseType: IntMathTypeTrait<'a>;

    fn new(value_ref: LLVMValueRef) -> Self;
}
