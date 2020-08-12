.section .text.entry
    .globl _start
_start:
    # t0 := 三级页表的虚拟地址
    lui     t0, %hi(boot_page_table_sv39)
    # t1 := 0xffffffff40000000 虚实映射偏移量 
    li      t1, 0xffffffffc0000000 - 0x80000000
    # t0 - 虚实映射偏移量 = 三级页表物理地址
    sub     t0, t0, t1
    # t0 >>= 12 变为三级页表的物理页号
    srli    t0, t0, 12

    # t1: 8 << 60, 设置 satp 的 MODE字段为 Sv39
    li      t1, 8 << 60
    # 将三级页表物理页号附加到 stap
    or      t0, t0, t1
    # 将新MODE|页表基址物理页号)覆盖到satp中
    csrw    satp, t0
    # 刷新TLB
    sfence.vma
    # 在虚拟内存空间中： 随意将 sp 设置为虚拟地址
    lui sp, %hi(bootstacktop)

    # 在虚拟内存空间中：随意跳转到虚拟地址
    # 跳转到 rust_main
    lui t0, %hi(rust_main)
    addi t0, t0, %lo(rust_main)
    jr t0

    .section .bss.stack
    # 由于要把页表放在一个页里面，因此必须12位对齐
    .align 12
    .global bootstack
bootstack:
    .space 4096 * 4
    .global bootstacktop
bootstacktop:

    .section .data
    .align 12   # page align
# 分配4KiB内存给预设的三级页表
boot_page_table_sv39:
    # 0xffffffff_c0000000 -> 0x80000000 (1G)
    # 前511个页表项均设置为 0， 因此 V=0 
    .zero 8 * 511
    # 设置最后一个页表项， PPN=0x80000,标志位 VRWXAD  均为 1
    .quad (0x80000 << 10) | 0xcf # VRWXAD