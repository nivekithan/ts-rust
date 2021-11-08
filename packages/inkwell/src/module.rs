use std::marker::PhantomData;

use llvm_sys::{
    core::{
        LLVMAddFunction, LLVMDisposeModule, LLVMGetModuleContext, LLVMGetNamedFunction,
        LLVMPrintModuleToString,
    },
    prelude::LLVMModuleRef,
};

use crate::{
    context::Context,
    enums::Linkage,
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
            "Containing Struct should have checked for null pointer"
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
        linkage: Option<Linkage>,
    ) -> FunctionValue {
        let c_name = to_c_str(name);

        unsafe {
            let value = FunctionValue::new(LLVMAddFunction(
                self.module,
                c_name.as_ptr(),
                ty.as_type_ref(),
            ));

            if let Some(linkage) = linkage {
                value.set_linkage(&linkage);
            };

            return value;
        }
    }

    pub fn get_string_representation(&self) -> LLVMString {
        unsafe { LLVMString::new(LLVMPrintModuleToString(self.module)) }
    }

    pub fn get_fn_value(&'a self, name: &str) -> FunctionValue<'a> {
        unsafe {
            let c_name = to_c_str(name);
            let value = LLVMGetNamedFunction(self.module, c_name.as_ptr());
            return FunctionValue::new(value);
        }
    }

    pub fn get_context(&'a self) -> Context {
        unsafe {
            let context_ref = LLVMGetModuleContext(self.module);
            return Context::new_without_drop(context_ref);
        }
    }
}

impl<'a> Drop for Module<'a> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.module);
        }
    }
}
