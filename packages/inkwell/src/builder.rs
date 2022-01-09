use std::marker::PhantomData;

use either::Either;
use llvm_sys::{
    core::{
        LLVMAddClause, LLVMBuildAlloca, LLVMBuildBr, LLVMBuildCall2, LLVMBuildCondBr,
        LLVMBuildFAdd, LLVMBuildFCmp, LLVMBuildFDiv, LLVMBuildFMul, LLVMBuildFNeg, LLVMBuildFPToSI,
        LLVMBuildFSub, LLVMBuildGEP2, LLVMBuildICmp, LLVMBuildInvoke2, LLVMBuildLandingPad,
        LLVMBuildLoad2, LLVMBuildRet, LLVMBuildRetVoid, LLVMBuildStore, LLVMBuildXor,
        LLVMDisposeBuilder, LLVMPositionBuilderAtEnd, LLVMSetCleanup,
    },
    prelude::{LLVMBuilderRef, LLVMValueRef},
};

use crate::{
    basic_block::BasicBlock,
    enums::{IntCompareOperator, RealCompareOperator},
    types::{
        enums::BasicTypeEnum,
        fn_type::FunctionType,
        int_type::IntType,
        traits::{AsTypeRef, BasicTypeTrait},
    },
    utils::to_c_str,
    values::{
        enums::BasicValueEnum,
        float_value::FloatValue,
        fn_value::FunctionValue,
        instruction_value::InstructionValue,
        int_value::IntValue,
        ptr_value::PointerValue,
        returned_value::ReturnedValue,
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

    pub fn position_at_end(&self, basic_block: &BasicBlock<'a>) {
        unsafe {
            return LLVMPositionBuilderAtEnd(self.builder, basic_block.basic_block);
        }
    }

    pub fn build_gep_2<T: AsTypeRef>(
        &self,
        ty: T,
        value: &PointerValue<'a>,
        indices: &[IntValue<'a>],
        name: &str,
    ) -> PointerValue<'a> {
        let c_name = to_c_str(name);

        unsafe {
            let mut index_values: Vec<LLVMValueRef> =
                indices.iter().map(|val| val.as_value_ref()).collect();

            let value = LLVMBuildGEP2(
                self.builder,
                ty.as_type_ref(),
                value.as_value_ref(),
                index_values.as_mut_ptr(),
                index_values.len() as u32,
                c_name.as_ptr(),
            );

            return PointerValue::new(value);
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

    pub fn build_fp_to_si(&self, float: FloatValue, int_type: IntType, name: &str) -> IntValue {
        let c_string = to_c_str(name);

        unsafe {
            let value = LLVMBuildFPToSI(
                self.builder,
                float.as_value_ref(),
                int_type.as_type_ref(),
                c_string.as_ptr(),
            );
            return IntValue::new(value);
        }
    }

    pub fn build_invoke_2(
        &self,
        fn_value: Either<&FunctionValue<'a>, &PointerValue<'a>>,
        args: &[BasicValueEnum<'a>],
        then_block: &BasicBlock<'a>,
        catch_block: &BasicBlock<'a>,
        mut name: &str,
    ) -> ReturnedValue<'a> {
        unsafe {
            if let Either::Left(fn_value) = fn_value {
                let fn_type = fn_value.get_type();
                let return_type = fn_type.get_return_type();
                if let BasicTypeEnum::VoidType(_) = &return_type {
                    name = ""
                };

                let c_name = to_c_str(name);
                let mut args: Vec<LLVMValueRef> = args.iter().map(|v| v.as_value_ref()).collect();

                let value = LLVMBuildInvoke2(
                    self.builder,
                    fn_type.as_type_ref(),
                    fn_value.as_value_ref(),
                    args.as_mut_ptr(),
                    args.len() as u32,
                    then_block.basic_block,
                    catch_block.basic_block,
                    c_name.as_ptr(),
                );

                return ReturnedValue::new(value);
            } else if let Either::Right(pointer_value) = fn_value {
                let is_valid_type = pointer_value.get_type().into_element_type().is_fn_type();

                if is_valid_type {
                    let fn_type = FunctionType::new(
                        pointer_value.get_type().into_element_type().as_type_ref(),
                    );

                    let return_type = fn_type.get_return_type();

                    if let BasicTypeEnum::VoidType(_) = return_type {
                        name = ""
                    };

                    let c_name = to_c_str(name);
                    let mut args: Vec<LLVMValueRef> =
                        args.iter().map(|v| v.as_value_ref()).collect();
                    let value = LLVMBuildInvoke2(
                        self.builder,
                        fn_type.as_type_ref(),
                        pointer_value.as_value_ref(),
                        args.as_mut_ptr(),
                        args.len() as u32,
                        then_block.basic_block,
                        catch_block.basic_block,
                        c_name.as_ptr(),
                    );

                    return ReturnedValue::new(value);
                } else {
                    todo!();
                }
            } else {
                todo!();
            }
        }
    }

    pub fn build_call2(
        &self,
        callable_value: Either<&FunctionValue<'a>, &PointerValue<'a>>,
        args: &[BasicValueEnum<'a>],
        mut name: &str,
    ) -> ReturnedValue<'a> {
        unsafe {
            if let Either::Left(fn_value) = callable_value {
                let fn_type = fn_value.get_type();

                let return_type = fn_type.get_return_type();

                if let BasicTypeEnum::VoidType(_) = return_type {
                    name = ""
                };

                let c_name = to_c_str(name);
                let mut args: Vec<LLVMValueRef> = args.iter().map(|v| v.as_value_ref()).collect();

                let value = LLVMBuildCall2(
                    self.builder,
                    fn_type.as_type_ref(),
                    fn_value.as_value_ref(),
                    args.as_mut_ptr(),
                    args.len() as u32,
                    c_name.as_ptr(),
                );

                return ReturnedValue::new(value);
            } else if let Either::Right(pointer_value) = callable_value {
                let is_valid_type = pointer_value.get_type().into_element_type().is_fn_type();

                if is_valid_type {
                    let fn_type = FunctionType::new(
                        pointer_value.get_type().into_element_type().as_type_ref(),
                    );

                    let return_type = fn_type.get_return_type();

                    if let BasicTypeEnum::VoidType(_) = return_type {
                        name = ""
                    };

                    let c_name = to_c_str(name);
                    let mut args: Vec<LLVMValueRef> =
                        args.iter().map(|v| v.as_value_ref()).collect();
                    let value = LLVMBuildCall2(
                        self.builder,
                        pointer_value.get_type().into_element_type().as_type_ref(),
                        pointer_value.as_value_ref(),
                        args.as_mut_ptr(),
                        args.len() as u32,
                        c_name.as_ptr(),
                    );

                    return ReturnedValue::new(value);
                } else {
                    todo!();
                }
            } else {
                unreachable!();
            }
        }
    }

    /*
     * TODO:
     * Learn more about personality functions
     *
     * */

    pub fn build_landing_pad(
        &self,
        exception_type: &BasicTypeEnum<'a>,
        personality_fn: &FunctionValue<'a>,
        clauses: &[BasicValueEnum<'a>],
        is_cleanup: bool,
        name: &str,
    ) -> BasicValueEnum {
        unsafe {
            let c_string = to_c_str(name);
            let num_clauses = clauses.len() as u32;

            let value = LLVMBuildLandingPad(
                self.builder,
                exception_type.as_type_ref(),
                personality_fn.as_value_ref(),
                num_clauses,
                c_string.as_ptr(),
            );

            for clause in clauses {
                LLVMAddClause(value, clause.as_value_ref());
            }

            LLVMSetCleanup(value, is_cleanup as i32);

            return BasicValueEnum::new(value);
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
