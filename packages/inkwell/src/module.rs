use std::marker::PhantomData;

use llvm_sys::{
    core::{LLVMAddFunction, LLVMDisposeModule, LLVMPrintModuleToString},
    prelude::LLVMModuleRef,
};

use crate::{
    types::{fn_type::FunctionType, traits::AsTypeRef},
    utils::{llvm_string::LLVMString, to_c_str},
    values::fn_value::FunctionValue,
};

pub struct Module<'a> {
    pub(crate) module: LLVMModuleRef,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Module<'a> {
    pub(crate) unsafe fn new(module: LLVMModuleRef) -> Self {
        debug_assert!(
            !module.is_null(),
            "Contaning Struct should haved checked for null pointer"
        );

        return Module {
            module,
            _marker: PhantomData,
        };
    }

    pub fn add_function(
        &self,
        name: &str,
        ty: FunctionType,
        _linkage: Option<()>,
    ) -> FunctionValue {
        let c_name = to_c_str(name);

        unsafe {
            return FunctionValue::new(LLVMAddFunction(
                self.module,
                c_name.as_ptr(),
                ty.as_type_ref(),
            ));
        }
    }

    pub fn print_to_string(&self) -> LLVMString {
        unsafe { LLVMString::new(LLVMPrintModuleToString(self.module)) }
    }
}

impl<'a> Drop for Module<'a> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.module);
        }
    }
}
