use std::convert::TryFrom;

use llvm_sys::{core::LLVMGetTypeKind, prelude::LLVMTypeRef, LLVMTypeKind};

use super::{
    array_type::ArrayType,
    float_type::FloatType,
    fn_type::FunctionType,
    int_type::IntType,
    ptr_type::PointerType,
    struct_type::StructType,
    traits::{AsTypeRef, BasicTypeTrait},
    void_type::VoidType,
};

#[derive(Debug, PartialEq, Clone)]
pub enum BasicTypeEnum<'a> {
    VoidType(VoidType<'a>),
    FloatType(FloatType<'a>),
    PointerType(PointerType<'a>),
    IntType(IntType<'a>),
    ArrayType(ArrayType<'a>),
    StructType(StructType<'a>),
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
            LLVMTypeKind::LLVMArrayTypeKind => BasicTypeEnum::ArrayType(ArrayType::new(type_)),
            LLVMTypeKind::LLVMStructTypeKind => BasicTypeEnum::StructType(StructType::new(type_)),

            t => unreachable!("unsupported type for BasicType generation : {:?}", t),
        }
    }

    pub fn fn_type(&self, param_types: &[BasicTypeEnum], variadic_arg: bool) -> FunctionType<'a> {
        match self {
            BasicTypeEnum::VoidType(type_) => type_.fn_type(param_types, variadic_arg),
            BasicTypeEnum::IntType(type_) => type_.fn_type(param_types, variadic_arg),
            BasicTypeEnum::FloatType(type_) => type_.fn_type(param_types, variadic_arg),
            BasicTypeEnum::PointerType(type_) => type_.fn_type(param_types, variadic_arg),

            _ => panic!("Cannot convert {:?} to fn_type", self),
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
            BasicTypeEnum::ArrayType(ty) => ty.as_type_ref(),
            BasicTypeEnum::StructType(ty) => ty.as_type_ref(),
        }
    }
}

impl<'a> BasicTypeTrait<'a> for BasicTypeEnum<'a> {}

// Defines the address space in which a global will be inserted.
//
// # Remarks
// See also: https://llvm.org/doxygen/NVPTXBaseInfo_8h_source.html
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum AddressSpace {
    Generic = 0,
    Global = 1,
    Shared = 3,
    Const = 4,
    Local = 5,
}

impl TryFrom<u32> for AddressSpace {
    type Error = ();

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(AddressSpace::Generic),
            1 => Ok(AddressSpace::Global),
            3 => Ok(AddressSpace::Shared),
            4 => Ok(AddressSpace::Const),
            5 => Ok(AddressSpace::Local),
            _ => Err(()),
        }
    }
}
