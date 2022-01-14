define void @syscallPrint_( double %rdi, i8* %rsi, double %size) {
entry:
     %rax_int = fptosi double %rax to i64
     %rdi_int = fptosi double %rdi to i64
     %size_int = fptosi double %size to i64
     call void asm sideeffect "syscall", "{rax},{rdi},{rsi},{rdx}"(i64 1, i64 %rdi_int, i8* %rsi, i64 %size_int)
     ret void
}