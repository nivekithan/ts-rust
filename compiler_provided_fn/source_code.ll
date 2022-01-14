define void @syscallPrint_( double %0, i8* %1, double %2) {
entry:
     %rdi_int = fptosi double %0 to i64
     %size_int = fptosi double %2 to i64
     call void asm sideeffect "syscall", "{rax},{rdi},{rsi},{rdx}"(i64 1, i64 %rdi_int, i8* %1, i64 %size_int)
     ret void
}