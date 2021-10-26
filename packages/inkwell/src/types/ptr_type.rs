use llvm_sys::{
    core::{LLVMGetElementType, LLVMGetTypeKind},
    prelude::LLVMTypeRef,
};

use super::{
    array_type::ArrayType,
    struct_type::StructType,
    traits::{AsTypeRef, BasicTypeTrait},
    Type,
};

pub struct PointerType<'a> {
    ptr_type: Type<'a>,
}

impl<'a> PointerType<'a> {
    pub(crate) unsafe fn new(ptr_type: LLVMTypeRef) -> Self {
        assert!(!ptr_type.is_null());

        return PointerType {
            ptr_type: Type::new(ptr_type),
        };
    }

    pub(crate) unsafe fn into_element_type(&self) -> Type<'a> {
        let element_type = LLVMGetElementType(self.as_type_ref());
        return Type::new(element_type);
    }

    pub fn into_array_type(&self) -> Result<ArrayType<'a>, String> {
        unsafe {
            let element_type = self.into_element_type();
            let is_array_type = element_type.is_array_type();

            if is_array_type {
                return Ok(ArrayType::new(element_type.as_type_ref()));
            } else {
                return Err(format!(
                    "Expected the pointer type to point to ArrayType but instead got {:?}",
                    LLVMGetTypeKind(element_type.as_type_ref())
                ));
            }
        }
    }

    pub fn into_struct_type(&self) -> Result<StructType<'a>, String> {
        unsafe {
            let element_type = self.into_element_type();
            let is_struct_type = element_type.is_struct_type();

            if is_struct_type {
                return Ok(StructType::new(element_type.as_type_ref()));
            } else {
                return Err(format!(
                    "Expected the pointer type to point to StructureType but instead got {:?}",
                    LLVMGetTypeKind(element_type.as_type_ref())
                ));
            }
        }
    }
}

impl<'a> AsTypeRef for PointerType<'a> {
    fn as_type_ref(&self) -> LLVMTypeRef {
        return self.ptr_type.as_type_ref();
    }
}

impl<'a> BasicTypeTrait<'a> for PointerType<'a> {}
