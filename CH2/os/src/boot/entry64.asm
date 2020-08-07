    .section .text.entry
    .globl _start
_start:
    la sp, bootstacktop # 修改栈指针寄存器sp为.bss .stack段的结束地址。高地址为栈顶
    call rust_main # 跳转到main.rs rust_main函数 

    .section .bss.stack
    .align 12
    .global bootstack
bootstack:
    .space 4096 * 4
    .global bootstacktop
bootstacktop: