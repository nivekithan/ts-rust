use super::{
    float_value::FloatValue,
    int_value::IntValue,
    ptr_value::PointerValue,
    traits::{AsValueRef, BasicValueTrait},
};
use llvm_sys::{
    core::{LLVMGetTypeKind, LLVMTypeOf},
    prelude::LLVMValueRef,
    LLVMTypeKind,
};

#[derive(Debug, Clone, PartialEq)]
pub enum BasicValueEnum<'a> {
    FloatValue(FloatValue<'a>),
    PointerValue(PointerValue<'a>),
    IntValue(IntValue<'a>),
}

impl<'a> BasicValueEnum<'a> {
    pub(crate) unsafe fn new(value: LLVMValueRef) -> Self {
        match LLVMGetTypeKind(LLVMTypeOf(value)) {
            LLVMTypeKind::LLVMDoubleTypeKind => BasicValueEnum::FloatValue(FloatValue::new(value)),
            LLVMTypeKind::LLVMPointerTypeKind => {
                BasicValueEnum::PointerValue(PointerValue::new(value))
            }
            LLVMTypeKind::LLVMIntegerTypeKind => BasicValueEnum::IntValue(IntValue::new(value)),

            _ => panic!("unsupported value kind for generation of BasicValue"),
        }
    }

    pub(crate) fn is_basic_value_enum(value_ref: LLVMValueRef) -> bool {
        unsafe {
            return match LLVMGetTypeKind(LLVMTypeOf(value_ref)) {
                LLVMTypeKind::LLVMDoubleTypeKind
                | LLVMTypeKind::LLVMPointerTypeKind
                | LLVMTypeKind::LLVMIntegerTypeKind => true,
                _ => false,
            };
        }
    }
}

impl<'a> AsValueRef for BasicValueEnum<'a> {
    fn as_value_ref(&self) -> LLVMValueRef {
        match self {
            BasicValueEnum::FloatValue(ty) => ty.as_value_ref(),
            BasicValueEnum::PointerValue(ty) => ty.as_value_ref(),
            BasicValueEnum::IntValue(ty) => ty.as_value_ref(),
        }
    }
}

impl<'a> BasicValueTrait<'a> for BasicValueEnum<'a> {}
