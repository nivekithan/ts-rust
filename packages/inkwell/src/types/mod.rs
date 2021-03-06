pub mod array_type;
pub mod enums;
pub mod float_type;
pub mod fn_type;
pub mod int_type;
pub mod ptr_type;
pub mod struct_type;
pub mod traits;
pub mod utils;
pub mod void_type;

use llvm_sys::{
    core::{LLVMArrayType, LLVMFunctionType, LLVMGetTypeKind, LLVMPointerType},
    prelude::LLVMTypeRef,
    LLVMTypeKind,
};
use std::marker::PhantomData;

use self::{
    array_type::ArrayType,
    enums::{AddressSpace, BasicTypeEnum},
    fn_type::FunctionType,
    ptr_type::PointerType,
    traits::AsTypeRef,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct Type<'a> {
    ty: LLVMTypeRef,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Type<'a> {
    pub(crate) unsafe fn new(ty: LLVMTypeRef) -> Self {
        assert!(!ty.is_null());

        return Type {
            ty,
            _marker: PhantomData,
        };
    }

    pub(crate) fn fn_type(
        self,
        param_types: &[BasicTypeEnum],
        variadic_arg: bool,
    ) -> FunctionType<'a> {
        let mut param_types: Vec<LLVMTypeRef> = param_types
            .iter()
            .map(|bt| return bt.as_type_ref())
            .collect();

        unsafe {
            return FunctionType::new(LLVMFunctionType(
                self.ty,
                param_types.as_mut_ptr(),
                param_types.len() as u32,
                variadic_arg as i32,
            ));
        }
    }

    pub(crate) fn array_type(self, size: u32) -> ArrayType<'a> {
        unsafe {
            return ArrayType::new(LLVMArrayType(self.ty, size));
        }
    }

    pub(crate) fn ptr_type(self, address_space: AddressSpace) -> PointerType<'a> {
        unsafe {
            return PointerType::new(LLVMPointerType(self.ty, address_space as u32));
        }
    }

    pub(crate) fn _is_void_type(&self) -> bool {
        unsafe {
            match LLVMGetTypeKind(self.ty) {
                LLVMTypeKind::LLVMVoidTypeKind => true,

                _ => false,
            }
        }
    }

    pub(crate) fn _is_double_type(&self) -> bool {
        unsafe {
            match LLVMGetTypeKind(self.ty) {
                LLVMTypeKind::LLVMDoubleTypeKind => true,

                _ => false,
            }
        }
    }

    pub(crate) fn is_array_type(&self) -> bool {
        unsafe {
            match LLVMGetTypeKind(self.ty) {
                LLVMTypeKind::LLVMArrayTypeKind => true,

                _ => false,
            }
        }
    }

    pub(crate) fn is_struct_type(&self) -> bool {
        unsafe {
            match LLVMGetTypeKind(self.ty) {
                LLVMTypeKind::LLVMStructTypeKind => true,

                _ => false,
            }
        }
    }

    pub(crate) fn is_fn_type(&self) -> bool {
        unsafe {
            match LLVMGetTypeKind(self.ty) {
                LLVMTypeKind::LLVMFunctionTypeKind => true,

                _ => false,
            }
        }
    }
}

impl<'a> AsTypeRef for Type<'a> {
    fn as_type_ref(&self) -> LLVMTypeRef {
        return self.ty;
    }
}
