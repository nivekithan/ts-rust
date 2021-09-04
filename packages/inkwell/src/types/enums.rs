use llvm_sys::{core::LLVMGetTypeKind, prelude::LLVMTypeRef, LLVMTypeKind};

use super::{
    float_type::FloatType,
    int_type::IntType,
    ptr_type::PointerType,
    traits::{AsTypeRef, BasicTypeTrait},
    void_type::VoidType,
};

pub enum BasicTypeEnum<'a> {
    VoidType(VoidType<'a>),
    FloatType(FloatType<'a>),
    PointerType(PointerType<'a>),
    IntType(IntType<'a>),
}

impl<'a> BasicTypeEnum<'a> {
    pub(crate) unsafe fn new(type_: LLVMTypeRef) -> Self {
        match LLVMGetTypeKind(type_) {
            LLVMTypeKind::LLVMVoidTypeKind => BasicTypeEnum::VoidType(VoidType::new(type_)),
            LLVMTypeKind::LLVMDoubleTypeKind => BasicTypeEnum::FloatType(FloatType::new(type_)),
            LLVMTypeKind::LLVMPointerTypeKind => {
                BasicTypeEnum::PointerType(PointerType::new(type_))
            }
            LLVMTypeKind::LLVMIntegerTypeKind => BasicTypeEnum::FloatType(FloatType::new(type_)),

            _ => unreachable!("unsupported type for BasicType generation"),
        }
    }
}

impl<'a> AsTypeRef for BasicTypeEnum<'a> {
    fn as_type_ref(&self) -> LLVMTypeRef {
        match self {
            BasicTypeEnum::VoidType(ty) => ty.as_type_ref(),
            BasicTypeEnum::FloatType(ty) => ty.as_type_ref(),
            BasicTypeEnum::PointerType(ty) => ty.as_type_ref(),
            BasicTypeEnum::IntType(ty) => ty.as_type_ref(),
        }
    }
}

impl<'a> BasicTypeTrait<'a> for BasicTypeEnum<'a> {}
