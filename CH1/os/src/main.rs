#![no_std]//禁止使用std，它依赖于操作系统
#![no_main]//不使用常规入口点
use core::panic::PanicInfo;//core库不需要操作系统支持
//实现panic，panic表明程序遇到不可恢复错误，只能被迫停止运行。
#[panic_handler]
fn panic(_info: &PanicInfo)->!{//!表示不返回，程序panic后陷入死循环

    loop{}
}
#[no_mangle]//确保生成_start函数，而非乱码。_start是大多系统默认入口
pub extern "C" fn _start() -> !{//是个C函数，不允许返回，直接被操作系统或bootloader直接调用。
    loop{}
}
/*
fn main() {
    //println!("Hello, world!");
}*/
/*
rust项目默认链接标准库std
若不使用依赖操作系统的库std，会出现三个错误：
1.println!未定义，输出到标准输出，这依赖了操作系统
2.panic_handler未定义
3.eh_personality未定义, eh即exception handling,
用于实现堆栈展开，与panic有关，错误层层抛出，解决见Cargo.toml
4.request 'start' lang_item.Rust程序使用运行时系统(runtime system)，导致main不是执行
的第一个函数。Rust先跳转到C runtime library中的crt0再跳到
start，C runtime和Rust runtime都需要标准库支持
5.linking with 'cc' failed: exit code: 1 该错误与C runtime相关，尽管C runtime入口点被覆盖，仍默认链接C runtime,需要一些C标准库(libc)内容。故同样禁用常规C启动例程
将cargo build 换成 cargo rustc -- -C link-arg=-nostartfiles
*/