use riscv::register::{
    scause::{
        self,
        Trap,
        Exception,
        Interrupt
    },
    sepc,
    stvec,
    sscratch,
    sstatus
};
use crate::timer::{
    TICKS,
    clock_set_next_event
};

use crate::context::TrapFrame;

global_asm!(include_str!("trap/trap.asm"));

pub fn init() {
    unsafe {
        extern "C" {
            fn __alltraps();
        }        
        sscratch::write(0);
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
        //设置sstatus的SIE位
	    sstatus::set_sie();
    }
    println!("++++ setup interrupt! ++++");
}

#[no_mangle]
pub fn rust_trap(tf: &mut TrapFrame) {
    //根据中断原因分类讨论
    match tf.scause.cause() {
        //断点中断
        Trap::Exception(Exception::Breakpoint) => breakpoint(&mut tf.sepc),
        //S态时钟中断
        Trap::Interrupt(Interrupt::SupervisorTimer) => super_timer(),
        _ => panic!("undefined trap!")
    }
}
//断点中断处理：输出断点地址并改变中断返回地址防止死循环
fn breakpoint(sepc: &mut usize) {
    println!("a breakpoint set @0x{:x}", sepc);
    *sepc += 2;
}
//S态时钟中断处理
fn super_timer() {
    clock_set_next_event();//设置下次时钟中断触发时间
    unsafe {
        TICKS += 1;//更新时钟中断触发计数
        if TICKS == 100 {
            TICKS = 0;
            println!("* 100 ticks *");
        }
    }
}