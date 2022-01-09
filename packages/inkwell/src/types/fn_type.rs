use std::os::raw::c_char;

use llvm_sys::{
    core::{LLVMGetInlineAsm, LLVMGetReturnType},
    prelude::LLVMTypeRef,
};

use crate::{enums::InlineAsmSyntax, utils::to_c_str, values::ptr_value::PointerValue};

use super::{
    enums::{AddressSpace, BasicTypeEnum},
    ptr_type::PointerType,
    traits::AsTypeRef,
    Type,
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct FunctionType<'a> {
    fn_type: Type<'a>,
}

impl<'a> FunctionType<'a> {
    pub(crate) unsafe fn new(fn_type: LLVMTypeRef) -> Self {
        assert!(!fn_type.is_null());

        return FunctionType {
            fn_type: Type::new(fn_type),
        };
    }

    pub fn get_return_type(&self) -> BasicTypeEnum<'a> {
        unsafe {
            let return_type = LLVMGetReturnType(self.as_type_ref());
            return BasicTypeEnum::new(return_type);
        }
    }
    pub fn ptr_type(&self, address_space: AddressSpace) -> PointerType<'a> {
        self.fn_type.ptr_type(address_space)
    }

    pub fn create_inline_asm(
        &self,
        assembly: &str,
        constraints: &str,
        side_effect: bool,
        align_stack: bool,
        dialect: InlineAsmSyntax,
    ) -> PointerValue {
        unsafe {
            let c_assembly = to_c_str(assembly);
            let c_constraints = to_c_str(constraints);

            let value = LLVMGetInlineAsm(
                self.as_type_ref(),
                c_assembly.as_ptr() as *mut c_char,
                assembly.len(),
                c_constraints.as_ptr() as *mut c_char,
                constraints.len(),
                side_effect as i32,
                align_stack as i32,
                dialect.convert_to_llvm_inline_asm_dialect(),
            );

            return PointerValue::new(value);
        }
    }
}

impl<'a> AsTypeRef for FunctionType<'a> {
    fn as_type_ref(&self) -> LLVMTypeRef {
        return self.fn_type.as_type_ref();
    }
}
