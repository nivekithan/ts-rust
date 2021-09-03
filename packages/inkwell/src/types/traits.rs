use llvm_sys::prelude::LLVMTypeRef;

use crate::values::traits::FloatMathValueTrait;

use super::enums::BasicTypeEnum;

pub trait AsTypeRef {
    fn as_type_ref(&self) -> LLVMTypeRef;
}

pub trait BasicTypeTrait<'a>: AsTypeRef {
    fn as_basic_type_enum(&self) -> BasicTypeEnum<'a> {
        unsafe { BasicTypeEnum::new(self.as_type_ref()) }
    }
}

pub trait FloatMathTypeTrait<'a>: BasicTypeTrait<'a> {
    type ValueType: FloatMathValueTrait<'a>;
}
