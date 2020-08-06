use riscv::register::{
    scause,
    sepc,
    stvec,
    sscratch,
    sstatus
};

pub fn init() {
    unsafe {
        sscratch::write(0);//上下文环境保存与恢复
        //设置stvec为direct，直接跳转到trap_handler
        stvec::write(trap_handler as usize, stvec::TrapMode::Direct);
        //sstatus::set_sie();
    }
    println!("++++ setup interrupt! ++++");
}
//输出中断原因及中断发生地址
fn trap_handler() -> ! {
    let cause = scause::read().cause();
    let epc = sepc::read();
    println!("trap: cause: {:?}, epc: 0x{:#x}", cause, epc);
    panic!("trap handled!");
}