use llvm_sys::{
    core::{LLVMContextCreate, LLVMContextDispose},
    prelude::*,
};

use crate::{
    basic_block::BasicBlock, builder::Builder, module::Module, values::double::LlvmDouble,
};

pub struct Context {
    llvm_ref: LLVMContextRef,
}

impl Context {
    pub fn new() -> Context {
        unsafe {
            let llvm_ref = LLVMContextCreate();
            return Context { llvm_ref };
        }
    }

    pub(crate) fn get_ref(&self) -> LLVMContextRef {
        return self.llvm_ref;
    }

    // Creates a module in this context
    pub fn create_module(&self, name: &str) -> Module {
        return Module::new(self, name);
    }

    pub fn create_basic_block(&self, name: &str) -> BasicBlock {
        return BasicBlock::new(self, name);
    }

    pub fn create_builder(&self) -> Builder {
        return Builder::new(self);
    }

    pub fn create_double(&self, num: f64) -> LlvmDouble {
        return LlvmDouble::new(self, num);
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.llvm_ref);
        }
    }
}
