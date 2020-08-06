独立可执行程序
------------
移除标准库依赖 #![no_std]
1.未实现println!
2.需要一个函数作为pnic_handler，在程序panic(不可恢复错误)时调用。
3.eh_personality，语义项(编译器内部需要的特殊函数或类型)，eh(exception handler)
`src/main.rs`  源程序
`Cargo.toml` 项目配置文件

移除runtime依赖
1.Rust语言首先跳转到C runtime library中的crt0设置C程序运行所需环境(创建堆栈，设置寄存器参数等),再跳转到Rust runtime，该入口点为start。再调用main进入主程序

cargo rustc -- -C link-arg=-nostartfiles