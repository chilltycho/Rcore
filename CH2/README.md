最小化内核
-------
目标三元组：cpu架构、供应商、操作系统、ABI(类似API，B代表二进制)
安装Rust，默认编译后可知性文件平台，利用rustc --version --verbose查看默认目标三元组
riscv64目标三元组：riscv64imac-unknown-none-elf
用命令cargo build --target riscv64imac-unknown-none-elf使对riscv64平台编译，编译结果见target/riscv64imac../debug
由于riscv64默认panic为abort，可将Cargo.toml中相关设置删去

`rust-objcopy target/riscv64imac-unknown-none-elf/debug/os --strip-all -O binary target/riscv64imac-unknown-none-elf/debug/kernel.bin` 将os文件(elf)转换为内核镜像kernel.bin，丢弃所有符号表及调试信息

`rust-objdump target/riscv64imac-unknown-none-elf/debug/os -d --arch-name=riscv64` 反汇编

利用链接脚本src/boot/linker64.ld指定内存布局。对OS内核，数据一般放在高地址空间，并配置编译选项 .cargo/config

.text段存放汇编代码
.rodata段存放只读数据，通常为常量
.data存放可读写数据，通常为全局变量
.bss存放初始化为0的可读写数据，与.data不同的是只需记录段大小及所在位置，不用记录里面数据
stack栈
heap堆，程序运行过程中内存的动态分配

C runtime入口点_start,希望它设置内核运行环境(kernel runtime)再开始执行内核的代码。OpenSBI固件实现了bootloader，从Machine模式切换到S模式，再跳转到固定地址0x80200000，开始执行内核代码。见boot/entry64.asm

这部分不能使用新版本rust，不然初始位置不为0x80200000