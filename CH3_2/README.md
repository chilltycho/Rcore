时钟中断
------
时钟中断：设定每隔一段时间硬件自动触发一次，在其对应的中断处理程序里，回到内核态，强制对用户态或内核态的程序进行打断、调度、监控，并进一步管理它们对资源使用情况。

riscv中的中断寄存器
S态的有sie（Supervisor Interrupt Enable,监管中断使能),sip(Supervisor Interrupt Pending,监管中断待处理)。处理的中断分为三种：
1.SI软件中断 2.TI时钟中断 3.EI外部中断
sie有一个STIE位（使能位），对应sip有一个STIP位，与时钟中断TI有关。当硬件决定触发时钟中断时，将STIP设置为1，一条指令执行完毕后，若STIP为1，且STIE为1，则进入中断处理程序。