global_asm!(include_str!("boot/entry64.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    crate::interrupt::init();
    crate::timer::init();//时钟初始化
    
    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    }
    panic!("end of rust_main");
    loop {}
}