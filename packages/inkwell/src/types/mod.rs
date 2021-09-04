pub mod enums;
pub mod float_type;
pub mod fn_type;
pub mod int_type;
pub mod ptr_type;
pub mod traits;
pub mod void_type;

use llvm_sys::{
    core::{LLVMFunctionType, LLVMGetTypeKind},
    prelude::LLVMTypeRef,
    LLVMTypeKind,
};
use std::marker::PhantomData;

use self::{enums::BasicTypeEnum, fn_type::FunctionType, traits::AsTypeRef};

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
        param_types: &[BasicTypeEnum<'a>],
        variadic_arg: bool,
    ) -> FunctionType<'a> {
        assert!(self.is_void_type() || self.is_double_type());

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

    pub(crate) fn is_void_type(&self) -> bool {
        unsafe {
            match LLVMGetTypeKind(self.ty) {
                LLVMTypeKind::LLVMVoidTypeKind => true,

                _ => false,
            }
        }
    }

    pub(crate) fn is_double_type(&self) -> bool {
        unsafe {
            match LLVMGetTypeKind(self.ty) {
                LLVMTypeKind::LLVMDoubleTypeKind => true,

                _ => false,
            }
        }
    }

    // pub(crate) fn is_fn_type(&self) -> bool {
    //     unsafe {
    //         match LLVMGetTypeKind(self.ty) {
    //             LLVMTypeKind::LLVMFunctionTypeKind => true,

    //             _ => false,
    //         }
    //     }
    // }
}

impl<'a> AsTypeRef for Type<'a> {
    fn as_type_ref(&self) -> LLVMTypeRef {
        return self.ty;
    }
}
