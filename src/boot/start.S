.section .text.boot
.globl _start
_start:
    csrr tp, mhartid
    li t0, 1024*4
    addi t1, tp, 1
    mul t0, t0, t1
    la sp, _stack
    add sp, sp, t0
call_kernel:
    mv a0, sp
    call kernel_init