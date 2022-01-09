use std::convert::TryInto;

use llvm_sys::{core::LLVMStructGetTypeAtIndex, prelude::LLVMTypeRef};

use super::{
    enums::{AddressSpace, BasicTypeEnum},
    ptr_type::PointerType,
    traits::{AsTypeRef, BasicTypeTrait},
    Type,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct StructType<'a> {
    struct_type: Type<'a>,
}

impl<'a> StructType<'a> {
    pub(crate) unsafe fn new(struct_type: LLVMTypeRef) -> Self {
        assert!(!struct_type.is_null());

        return StructType {
            struct_type: Type::new(struct_type),
        };
    }

    pub fn get_field_type(&self, index: usize) -> BasicTypeEnum {
        unsafe {
            let type_ref = LLVMStructGetTypeAtIndex(self.as_type_ref(), index.try_into().unwrap());
            return BasicTypeEnum::new(type_ref);
        }
    }

    pub fn ptr_type(&self, address_space: AddressSpace) -> PointerType<'a> {
        return self.struct_type.ptr_type(address_space);
    }
}

impl<'a> AsTypeRef for StructType<'a> {
    fn as_type_ref(&self) -> LLVMTypeRef {
        return self.struct_type.as_type_ref();
    }
}

impl<'a> BasicTypeTrait<'a> for StructType<'a> {}
