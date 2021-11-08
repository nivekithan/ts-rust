use llvm_sys::{core::LLVMPrintTypeToString, prelude::LLVMTypeRef};

use crate::utils::llvm_string::LLVMString;

pub unsafe fn print_type_ref(type_ref: LLVMTypeRef) {
    assert!(!type_ref.is_null());
    let s = LLVMPrintTypeToString(type_ref);
    let llvm_string = LLVMString::new(s);
    println!("{}", llvm_string.to_string());
}
