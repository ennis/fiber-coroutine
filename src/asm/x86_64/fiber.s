// Mark stack as non-executable
#if defined(__linux__) && defined(__ELF__)
.section .note.GNU-stack, "", @progbits
#endif

.text
.globl GetFiberData
.globl GetCurrentFiber

GetFiberData:
	movq %gs:0x20,%rax
	movq (%rax),%rax
	ret

GetCurrentFiber:
	movq %gs:0x20,%rax
	ret
