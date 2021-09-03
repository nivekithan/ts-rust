use std::marker::PhantomData;

use llvm_sys::{
    core::{
        LLVMBuildAlloca, LLVMBuildFAdd, LLVMBuildFDiv, LLVMBuildFMul, LLVMBuildFNeg, LLVMBuildFSub,
        LLVMBuildRet, LLVMBuildRetVoid, LLVMBuildStore, LLVMDisposeBuilder,
        LLVMPositionBuilderAtEnd,
    },
    prelude::LLVMBuilderRef,
};

use crate::{
    basic_block::BasicBlock,
    types::traits::BasicTypeTrait,
    utils::to_c_str,
    values::{
        instruction_value::InstructionValue,
        ptr_value::PointerValue,
        traits::{AsValueRef, BasicValueTrait, FloatMathValueTrait},
    },
};

pub struct Builder<'a> {
    pub(crate) builder: LLVMBuilderRef,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Builder<'a> {
    pub(crate) unsafe fn new(builder: LLVMBuilderRef) -> Self {
        assert!(!builder.is_null());

        return Builder {
            builder,
            _marker: PhantomData,
        };
    }

    pub fn position_at_end(&self, basic_block: BasicBlock<'a>) {
        unsafe {
            return LLVMPositionBuilderAtEnd(self.builder, basic_block.basic_block);
        }
    }

    pub fn build_alloca<T: BasicTypeTrait<'a>>(&self, ty: T, name: &str) -> PointerValue<'a> {
        let c_name = to_c_str(name);
        unsafe {
            let ptr_value = LLVMBuildAlloca(self.builder, ty.as_type_ref(), c_name.as_ptr());
            return PointerValue::new(ptr_value);
        }
    }

    pub fn build_store<V: BasicValueTrait<'a>>(
        &self,
        ptr: PointerValue<'a>,
        value: V,
    ) -> InstructionValue<'a> {
        unsafe {
            let value = LLVMBuildStore(self.builder, value.as_value_ref(), ptr.as_value_ref());
            return InstructionValue::new(value);
        }
    }

    pub fn build_float_add<T: FloatMathValueTrait<'a>>(&self, lhs: T, rhs: T, name: &str) -> T {
        let c_name = to_c_str(name);

        unsafe {
            let value = LLVMBuildFAdd(
                self.builder,
                lhs.as_value_ref(),
                rhs.as_value_ref(),
                c_name.as_ptr(),
            );
            return T::new(value);
        }
    }

    pub fn build_float_sub<T: FloatMathValueTrait<'a>>(&self, lhs: T, rhs: T, name: &str) -> T {
        let c_name = to_c_str(name);

        unsafe {
            let value = LLVMBuildFSub(
                self.builder,
                lhs.as_value_ref(),
                rhs.as_value_ref(),
                c_name.as_ptr(),
            );
            return T::new(value);
        }
    }

    pub fn build_float_mul<T: FloatMathValueTrait<'a>>(&self, lhs: T, rhs: T, name: &str) -> T {
        let c_name = to_c_str(name);

        unsafe {
            let value = LLVMBuildFMul(
                self.builder,
                lhs.as_value_ref(),
                rhs.as_value_ref(),
                c_name.as_ptr(),
            );
            return T::new(value);
        }
    }

    pub fn build_float_div<T: FloatMathValueTrait<'a>>(&self, lhs: T, rhs: T, name: &str) -> T {
        let c_name = to_c_str(name);

        unsafe {
            let value = LLVMBuildFDiv(
                self.builder,
                lhs.as_value_ref(),
                rhs.as_value_ref(),
                c_name.as_ptr(),
            );
            return T::new(value);
        }
    }

    pub fn build_float_neg<T: FloatMathValueTrait<'a>>(&self, value: T, name: &str) -> T {
        let c_name = to_c_str(name);

        unsafe {
            let value = LLVMBuildFNeg(self.builder, value.as_value_ref(), c_name.as_ptr());
            return T::new(value);
        }
    }

    pub fn build_return(&self, value: Option<&dyn BasicValueTrait<'a>>) -> InstructionValue<'a> {
        unsafe {
            let value = value.map_or_else(
                || LLVMBuildRetVoid(self.builder),
                |value| LLVMBuildRet(self.builder, value.as_value_ref()),
            );

            return InstructionValue::new(value);
        }
    }
}

impl<'a> Drop for Builder<'a> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.builder);
        }
    }
}
