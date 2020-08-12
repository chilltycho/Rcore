use riscv::register::{
    sstatus::Sstatus,
    scause::Scause,
};
use riscv::register::sstatus;
use core::mem::zeroed;

#[repr(C)]
pub struct TrapFrame {
    pub x: [usize; 32], // General registers
    pub sstatus: Sstatus, // Supervisor Status Register
    pub sepc: usize, // Supervisor exception program counter
    pub stval: usize, // Supervisor trap value
    pub scause: Scause, // Scause register: record the cause of exception/interrupt/trap
}
//描述别切换出去的线程的状态
#[repr(C)]
pub struct Context {
    pub content_addr: usize,
}

//通过switch.asm实现该函数
impl Context {
    #[naked]//不要给函数插入prologue或epilogue
    #[inline(never)]//不要内联该函数
    pub unsafe extern "C" fn switch(&mut self, target: &mut Context) {
        llvm_asm!(include_str!("process/switch.asm") :::: "volatile");
    }

	pub fn null() -> Context {
        Context { content_addr: 0, }
    }

	pub unsafe fn new_kernel_thread(
        entry: usize,
        kstack_top: usize,
        satp: usize
        ) -> Context {

        ContextContent::new_kernel_thread(entry, kstack_top, satp).push_at(kstack_top)
    }

    pub unsafe fn append_initial_arguments(&self, args: [usize; 3]) {
        let contextContent = &mut *(self.content_addr as *mut ContextContent);
        contextContent.tf.x[10] = args[0];
        contextContent.tf.x[11] = args[1];
        contextContent.tf.x[12] = args[2];
    }

}
//线程在栈上保存的内容
#[repr(C)]
pub struct ContextContent {
    pub ra: usize,
    satp: usize,
    s: [usize; 12],
    tf: TrapFrame,//中断帧，用来线程初始化
}

extern "C" {
	fn __trapret();
}

impl ContextContent {
    fn new_kernel_thread(//为新内核线程构造栈上的初始状态信息，入口点为entry,内核栈栈顶地址为kstack_top
        entry: usize,
        kstack_top: usize,
        satp: usize,//页表
        ) -> ContextContent {

        let mut content = ContextContent {
            ra: __trapret as usize,
            satp,
            s: [0; 12],
            tf: {
                let mut tf: TrapFrame = unsafe { zeroed() };
                tf.x[2] = kstack_top;
                tf.sepc = entry;
                tf.sstatus = sstatus::read();
                tf.sstatus.set_spp(sstatus::SPP::Supervisor);
                tf.sstatus.set_spie(true);
                tf.sstatus.set_sie(false);
                tf
            }
        };
        content
    }

	unsafe fn push_at(self, stack_top: usize) -> Context {
        let ptr = (stack_top as *mut ContextContent).sub(1);
        *ptr = self;
        Context { content_addr: ptr as usize }
    }
}
