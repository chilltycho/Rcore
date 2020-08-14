# a0 a1分别保存"当前线程栈顶地址"所在地址，"要切换到的线程栈顶地址"的地址
.equ XLENB, 8
.macro Load a1, a2 
    ld \a1, \a2*XLENB(sp)
.endm
.macro Store a1, a2 
    sd \a1, \a2*XLENB(sp)
.endm
    addi sp, sp, -14*XLENB # 入栈，在当前栈上分配空间保存当前CPU状态
    sd sp, 0(a0) # 更新a0
    Store ra, 0 # 依次保存各寄存器的值
    Store s0, 2
    Store s1, 3
    Store s2, 4
	Store s3, 5
	Store s4, 6
	Store s5, 7
	Store s6, 8
	Store s7, 9
	Store s8, 10
	Store s9, 11
    Store s10, 12
    Store s11, 13
    csrr s11, satp
    Store s11, 1 # 当前线程状态保存完毕
# 准备恢复到“要切换到的线程"
    ld sp, 0(a1) # 读取a1并换栈
    Load s11, 1
    csrw satp, s11 # 恢复页表寄存器
    sfence.vma # 刷新 TLB
    Load ra, 0
    Load s0, 2
    Load s1, 3
    Load s2, 4
	Load s3, 5
	Load s4, 6
	Load s5, 7
	Load s6, 8
	Load s7, 9
	Load s8, 10
	Load s9, 11
    Load s10, 12
    Load s11, 13 # 各寄存器均被恢复
    addi sp, sp, 14*XLENB # 出栈，在当前栈上回收用来保存线程状态的内存
# 将a1修改为0，防止别人switch到它，把它的栈修改
    sd zero, 0(a1)
    ret