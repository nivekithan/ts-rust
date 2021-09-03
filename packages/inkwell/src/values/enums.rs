use super::{
    float_value::FloatValue,
    ptr_value::PointerValue,
    traits::{AsValueRef, BasicValueTrait},
};
use llvm_sys::{
    core::{LLVMGetTypeKind, LLVMTypeOf},
    prelude::LLVMValueRef,
    LLVMTypeKind,
};

pub enum BasicValueEnum<'a> {
    FloatValue(FloatValue<'a>),
    PointerValue(PointerValue<'a>),
}

impl<'a> BasicValueEnum<'a> {
    pub(crate) unsafe fn new(value: LLVMValueRef) -> Self {
        match LLVMGetTypeKind(LLVMTypeOf(value)) {
            LLVMTypeKind::LLVMDoubleTypeKind => BasicValueEnum::FloatValue(FloatValue::new(value)),
            LLVMTypeKind::LLVMPointerTypeKind => {
                BasicValueEnum::PointerValue(PointerValue::new(value))
            }

            _ => panic!("unsupported value kind for generation of BasicValue"),
        }
    }
}

impl<'a> AsValueRef for BasicValueEnum<'a> {
    fn as_value_ref(&self) -> LLVMValueRef {
        match self {
            BasicValueEnum::FloatValue(ty) => ty.as_value_ref(),
            BasicValueEnum::PointerValue(ty) => ty.as_value_ref(),
        }
    }
}

impl<'a> BasicValueTrait<'a> for BasicValueEnum<'a> {}
