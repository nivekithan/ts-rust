use llvm_sys::prelude::LLVMTypeRef;

use crate::values::traits::{FloatMathValueTrait, IntMathValueTrait};

use super::{enums::BasicTypeEnum, utils::print_type_ref};

pub trait AsTypeRef {
    fn as_type_ref(&self) -> LLVMTypeRef;
}

pub trait BasicTypeTrait<'a>: AsTypeRef {
    fn as_basic_type_enum(&self) -> BasicTypeEnum<'a> {
        unsafe { BasicTypeEnum::new(self.as_type_ref()) }
    }

    fn print(&self) {
        let ref_ = self.as_type_ref();
        unsafe { print_type_ref(ref_) };
    }
}
pub trait FloatMathTypeTrait<'a>: BasicTypeTrait<'a> {
    type ValueType: FloatMathValueTrait<'a>;
}

pub trait IntMathTypeTrait<'a>: BasicTypeTrait<'a> {
    type ValueType: IntMathValueTrait<'a>;
}
