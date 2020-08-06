use riscv::register::{
    scause,
    sepc,
    stvec,
    sscratch,
    sstatus
};
//引入TrapFrame结构体
use crate::context::TrapFrame;
//载入trap.asm
global_asm!(include_str!("trap/trap.asm"));

pub fn init() {
    unsafe {
        extern "C" {
            //中断处理总入口
            fn __alltraps();
        }        
        // 经过上面的分析，由于现在是在内核态
        // 我们要把 sscratch 初始化为 0
        sscratch::write(0);
        // 仍使用 Direct 模式
        // 将中断处理总入口设置为 __alltraps
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
        //sstatus::set_sie();
    }
    println!("++++ setup interrupt! ++++");
}

#[no_mangle]
pub fn rust_trap(tf: &mut TrapFrame) {
    println!("rust_trap!");
    let cause = scause::read().cause();
    let epc = sepc::read();
    println!("trap: cause: {:?}, epc: 0x{:#x}", cause, epc);
    // 触发中断时，硬件会将 sepc 设置为触发中断指令的地址
    // 而中断处理结束，使用 sret 返回时也会跳转到 sepc 处
    // 于是我们又要执行一次那条指令，触发中断，无限循环下去
    // 而我们这里是断点中断，只想这个中断触发一次
    // 因此我们将中断帧内的 sepc 字段设置为触发中断指令下一条指令的地址，即中断结束后跳过这条语句
    // 由于 riscv64 的每条指令都是 32 位，4 字节，因此将地址+ 4 即可
    // 这样在 RESTORE_ALL 时，这个修改后的 sepc 字段就会被 load 到 sepc 寄存器中
    // 使用 sret 返回时就会跳转到 ebreak 的下一条指令了
    tf.sepc += 2;//实际ebreak为2字节，不是4字节
}