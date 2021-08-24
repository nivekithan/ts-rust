use llvm_sys::{
    core::{
        LLVMAddFunction, LLVMDisposeModule, LLVMGetModuleContext, LLVMModuleCreateWithNameInContext,
    },
    prelude::*,
};

use crate::{
    context::Context,
    function::{LlvmFunction, LlvmFunctionType},
    types::LlvmType,
    utils::convert_to_c_string,
};

pub struct Module {
    llvm_ref: LLVMModuleRef,
}

impl Module {
    pub fn new(context: &Context, name: &str) -> Module {
        let c_string = convert_to_c_string(name).unwrap();

        unsafe {
            let llvm_ref = LLVMModuleCreateWithNameInContext(c_string, context.get_ref());
            return Module { llvm_ref };
        }
    }

    pub(crate) fn get_ref(&self) -> LLVMModuleRef {
        return self.llvm_ref;
    }

    pub(crate) fn get_context_ref(&self) -> LLVMContextRef {
        unsafe {
            let context_ref = LLVMGetModuleContext(self.get_ref());
            return context_ref;
        }
    }

    pub(crate) fn add_function_of_type(
        &self,
        name: &str,
        function_type: &LlvmFunctionType,
    ) -> LlvmFunction {
        let c_string = convert_to_c_string(name).unwrap();

        unsafe {
            let value = LLVMAddFunction(self.get_ref(), c_string, function_type.get_ref());
            return LlvmFunction::new(self.get_context_ref(), value);
        }
    }

    pub fn add_function(
        &self,
        name: &str,
        param_types: &Vec<LlvmType>,
        return_type: &LlvmType,
    ) -> Result<LlvmFunction, String> {
        let function_type = LlvmFunctionType::new(param_types, return_type)?;
        return Ok(self.add_function_of_type(name, &function_type));
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.llvm_ref);
        }
    }
}
