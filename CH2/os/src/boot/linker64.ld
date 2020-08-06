OUTPUT_ARCH(riscv)/*指定框架*/
ENTRY(_start)/*指定入口点*/
/*OpenSBI将自身放在0x80000000,初始化后跳到x080200000*/
BASE_ADDRESS = 0x80200000;/*从该地址开始向下放置各个段：.text,.rodata,.data,.stack,.bss*/

SECTIONS
{
    /*output section:{input section list}*/
    . = BASE_ADDRESS;/*.表示当前地址*/
    start = .;

    .text : {
        stext = .;/*记下开始地址*/
        *(.text.entry)/*  *()将各个文件中所有括号内要求的输入段放在当前位置*/
        *(.text .text.*)
        . = ALIGN(4K);
        etext = .;/*记下结尾地址*/
    }

    .rodata : {
        srodata = .;
        *(.rodata .rodata.*)
        . = ALIGN(4K);
        erodata = .;
    }

    .data : {
        sdata = .;
        *(.data .data.*)
        edata = .;
    }

    .stack : {
        *(.bss.stack)
    }

    .bss : {
        sbss = .;
        *(.bss .bss.*)
        ebss = .;
    }

    PROVIDE(end = .);
}