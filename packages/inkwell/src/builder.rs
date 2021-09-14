use std::marker::PhantomData;

use llvm_sys::{
    core::{
        LLVMBuildAlloca, LLVMBuildBr, LLVMBuildCondBr, LLVMBuildFAdd, LLVMBuildFCmp, LLVMBuildFDiv,
        LLVMBuildFMul, LLVMBuildFNeg, LLVMBuildFSub, LLVMBuildICmp, LLVMBuildLoad2, LLVMBuildRet,
        LLVMBuildRetVoid, LLVMBuildStore, LLVMBuildXor, LLVMDisposeBuilder,
        LLVMPositionBuilderAtEnd,
    },
    prelude::LLVMBuilderRef,
};

use crate::{
    basic_block::BasicBlock,
    enums::{IntCompareOperator, RealCompareOperator},
    types::{
        enums::BasicTypeEnum,
        traits::{AsTypeRef, BasicTypeTrait},
    },
    utils::to_c_str,
    values::{
        enums::BasicValueEnum,
        instruction_value::InstructionValue,
        int_value::IntValue,
        ptr_value::PointerValue,
        traits::{AsValueRef, BasicValueTrait, FloatMathValueTrait, IntMathValueTrait},
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

    pub fn build_load(
        &self,
        ptr: PointerValue<'a>,
        ty: BasicTypeEnum,
        name: &str,
    ) -> BasicValueEnum<'a> {
        unsafe {
            let c_name = to_c_str(name);
            let value = LLVMBuildLoad2(
                self.builder,
                ty.as_type_ref(),
                ptr.as_value_ref(),
                c_name.as_ptr(),
            );
            return BasicValueEnum::new(value);
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

    pub fn build_xor<T: IntMathValueTrait<'a>>(&self, lhs: T, rhs: T, name: &str) -> T {
        let c_name = to_c_str(name);

        unsafe {
            let value = LLVMBuildXor(
                self.builder,
                lhs.as_value_ref(),
                rhs.as_value_ref(),
                c_name.as_ptr(),
            );
            return T::new(value);
        }
    }

    pub fn build_int_compare<T: IntMathValueTrait<'a>>(
        &self,
        operator: IntCompareOperator,
        lhs: T,
        rhs: T,
        name: &str,
    ) -> IntValue {
        let c_name = to_c_str(name);

        unsafe {
            let value = LLVMBuildICmp(
                self.builder,
                operator.convert_llvm_int_predicate(),
                lhs.as_value_ref(),
                rhs.as_value_ref(),
                c_name.as_ptr(),
            );
            return IntValue::new(value);
        }
    }
    pub fn build_float_compare<T: FloatMathValueTrait<'a>>(
        &self,
        operator: RealCompareOperator,
        lhs: T,
        rhs: T,
        name: &str,
    ) -> IntValue {
        let c_name = to_c_str(name);

        unsafe {
            let value = LLVMBuildFCmp(
                self.builder,
                operator.convert_to_llvm_real_predicate(),
                lhs.as_value_ref(),
                rhs.as_value_ref(),
                c_name.as_ptr(),
            );
            return IntValue::new(value);
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

    pub fn build_unconditional_branch(
        &self,
        destination_block: &BasicBlock<'a>,
    ) -> InstructionValue<'a> {
        unsafe {
            let value = LLVMBuildBr(self.builder, destination_block.basic_block);
            return InstructionValue::new(value);
        }
    }

    pub fn build_conditional_branch(
        &self,
        condition: IntValue<'a>,
        if_block: &BasicBlock<'a>,
        else_block: &BasicBlock<'a>,
    ) -> InstructionValue<'a> {
        unsafe {
            let value = LLVMBuildCondBr(
                self.builder,
                condition.as_value_ref(),
                if_block.basic_block,
                else_block.basic_block,
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
