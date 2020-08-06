中断
------
rv64权限模式：M，S内核，U用户
1.M模式是RISC-V中hart(hardware thread,硬件线程)可执行的最高权限模式，默认下，所有异常控制权都被移交到M模式的异常处理程序
2.S（监管者）模式，支持基于页面的虚拟内存机制。M模式可将异常导向S模式，也可选择性将中断和同步异常直接交给S模式处理，完全绕过M模式

rv64中断相关寄存器：
1.sepc(exception program counter)，记录触发中断的那条指令的地址
2.scause，记录中断发生的原因，是否为外部中断。外部中断一般为外设，内部中断一般为硬件错误，如断电，除0
3.stvec，设置如何寻找S态中断处理程序的起始地址，保存了中断向量表基址BASE和模式MODE。
  MODE=0，Direct模式，无论中断因何发生直接跳转到基址pc<-BASE。
  MODE=1，Vectored模式，pc<-BASE+4*cause。只需将中断处理程序放在正确位置，并设置好stvec,遇到中断时硬件根据中断原因自动跳转到对应中断处理程序
4.sstatus,S态控制状态寄存器。保存全局中断使能标志及其他状态，可设置此寄存器来使能中断

中断相关指令：
ecall(environment call):当在S态执行该命令时，触发ecall-from-s-mode-exception异常，进入M模式中的中断处理流程(如设置定时器等)。在U态执行时，触发异常进入S模式中断处理流程(常用来进行系统调用)。
sret,用于S态中断返回到U态，实际作用为pc<-sepc
ebreak,触发一个断点中断从而进入中断处理流程
mret,M态中断返回到S态或U态，pc<-mepc

手动触发断点中断：
1.OS在初始化时，设置好中断处理程序起始地址，并使能。引入对寄存器操作的库，见Cargo.toml dependecies。
2.设置中断处理程序起始地址，lib.rs添加mod interrupt,见interrupt.rs
3.init.rs手动触发中断

可能出现错误结果：
++++setup interrupt!++++
++++setup interrupt!++++

qemu模拟的riscv不断重新启动，因为异常处理入口必须按照四字节对齐，设置stvec时，最低两位地址被置零，发生异常时可能直接跳转到异常处理程序的第一条指令中间。