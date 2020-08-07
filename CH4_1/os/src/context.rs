use riscv::register::{
    sstatus::Sstatus,
    scause::Scause,
};

#[repr(C)]//结构体按照C语言标准进行内存布局，有序，保证汇编正确设置
pub struct TrapFrame {
    pub x: [usize; 32], // 保存32个通用寄存器
    pub sstatus: Sstatus, // rv64的4个中断相关寄存器
    pub sepc: usize, // 
    pub stval: usize, // 
    pub scause: Scause, // 
}