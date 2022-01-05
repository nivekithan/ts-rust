use llvm_sys::{LLVMInlineAsmDialect, LLVMIntPredicate, LLVMRealPredicate};

pub enum IntCompareOperator {
    Equal,
    NotEqual,
}

impl IntCompareOperator {
    pub(crate) fn convert_llvm_int_predicate(&self) -> LLVMIntPredicate {
        match self {
            IntCompareOperator::Equal => LLVMIntPredicate::LLVMIntEQ,
            IntCompareOperator::NotEqual => LLVMIntPredicate::LLVMIntNE,
        }
    }
}

pub enum RealCompareOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

impl RealCompareOperator {
    pub(crate) fn convert_to_llvm_real_predicate(&self) -> LLVMRealPredicate {
        match self {
            RealCompareOperator::Equal => LLVMRealPredicate::LLVMRealOEQ,
            RealCompareOperator::NotEqual => LLVMRealPredicate::LLVMRealONE,
            RealCompareOperator::GreaterThan => LLVMRealPredicate::LLVMRealOGT,
            RealCompareOperator::GreaterThanOrEqual => LLVMRealPredicate::LLVMRealOGE,
            RealCompareOperator::LessThan => LLVMRealPredicate::LLVMRealOLT,
            RealCompareOperator::LessThanOrEqual => LLVMRealPredicate::LLVMRealOLE,
        }
    }
}

pub enum Linkage {
    External,
}

pub enum InlineAsmSyntax {
    Att,
    Intel,
}

impl InlineAsmSyntax {
    pub(crate) fn convert_to_llvm_inline_asm_dialect(&self) -> LLVMInlineAsmDialect {
        match self {
            InlineAsmSyntax::Att => return LLVMInlineAsmDialect::LLVMInlineAsmDialectATT,
            InlineAsmSyntax::Intel => return LLVMInlineAsmDialect::LLVMInlineAsmDialectIntel,
        }
    }
}
