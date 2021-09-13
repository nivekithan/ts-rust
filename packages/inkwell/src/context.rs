use llvm_sys::{core::{LLVMAppendBasicBlockInContext, LLVMContextCreate, LLVMContextDispose, LLVMCreateBuilderInContext, LLVMDoubleTypeInContext, LLVMInt1TypeInContext, LLVMInt64TypeInContext, LLVMModuleCreateWithNameInContext, LLVMVoidTypeInContext}, prelude::LLVMContextRef};

use crate::{
    basic_block::BasicBlock,
    builder::Builder,
    module::Module,
    types::{float_type::FloatType, int_type::IntType, void_type::VoidType},
    utils::to_c_str,
    values::{fn_value::FunctionValue, traits::AsValueRef},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Context {
    pub(crate) context: LLVMContextRef,
}

impl Context {
    pub(crate) unsafe fn new(context: LLVMContextRef) -> Self {
        assert!(!context.is_null());

        return Context { context };
    }

    pub fn create() -> Self {
        unsafe { return Context::new(LLVMContextCreate()) }
    }

    pub fn create_module(&self, name: &str) -> Module {
        let c_string = to_c_str(name);

        unsafe {
            return Module::new(LLVMModuleCreateWithNameInContext(
                c_string.as_ptr(),
                self.context,
            ));
        }
    }

    pub fn i1_type(&self) -> IntType {
        unsafe {
            return IntType::new(LLVMInt1TypeInContext(self.context));
        }
    }

    pub fn i64_type(&self) -> IntType {
        unsafe {
            return IntType::new(LLVMInt64TypeInContext(self.context));
        }
    }

    pub fn f64_type(&self) -> FloatType {
        unsafe { return FloatType::new(LLVMDoubleTypeInContext(self.context)) }
    }

    pub fn void_type(&self) -> VoidType {
        unsafe { return VoidType::new(LLVMVoidTypeInContext(self.context)) }
    }

    pub fn append_basic_block(&self, function: FunctionValue, name: &str) -> BasicBlock {
        let c_name = to_c_str(name);

        unsafe {
            return BasicBlock::new(LLVMAppendBasicBlockInContext(
                self.context,
                function.as_value_ref(),
                c_name.as_ptr(),
            ));
        }
    }

    pub fn create_builder(&self) -> Builder {
        unsafe {
            return Builder::new(LLVMCreateBuilderInContext(self.context));
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.context);
        }
    }
}
