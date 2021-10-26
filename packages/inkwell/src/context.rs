use llvm_sys::{
    core::{
        LLVMAppendBasicBlockInContext, LLVMContextCreate, LLVMContextDispose,
        LLVMCreateBuilderInContext, LLVMDoubleTypeInContext, LLVMInt1TypeInContext,
        LLVMInt32TypeInContext, LLVMInt64TypeInContext, LLVMInt8TypeInContext,
        LLVMModuleCreateWithNameInContext, LLVMStructTypeInContext, LLVMVoidTypeInContext,
    },
    prelude::{LLVMContextRef, LLVMTypeRef},
};

use crate::{
    basic_block::BasicBlock,
    builder::Builder,
    module::Module,
    types::{
        enums::BasicTypeEnum, float_type::FloatType, int_type::IntType, struct_type::StructType,
        traits::AsTypeRef, void_type::VoidType,
    },
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

    pub fn i8_type(&self) -> IntType {
        unsafe {
            return IntType::new(LLVMInt8TypeInContext(self.context));
        }
    }

    pub fn i32_type(&self) -> IntType {
        unsafe {
            return IntType::new(LLVMInt32TypeInContext(self.context));
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

    // TODO: I have no idea on what does packed means, should know about it
    pub fn struct_type(&self, field_types: &Vec<BasicTypeEnum>, packed: bool) -> StructType {
        let mut field_types: Vec<LLVMTypeRef> =
            field_types.iter().map(|val| val.as_type_ref()).collect();
        unsafe {
            StructType::new(LLVMStructTypeInContext(
                self.context,
                field_types.as_mut_ptr(),
                field_types.len() as u32,
                packed as i32,
            ))
        }
    }

    pub fn append_basic_block(&self, function: &FunctionValue, name: &str) -> BasicBlock {
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
