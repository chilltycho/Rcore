动态内存分配
----------
内核需要动态内存分配，典型应用场景有：
1.Box<T>，和new,malloc有相同功能
2.引用计数Rc<T>,原子引用计数Arc<T>。
3.数据结构，Vec,HashMap等

为在内核中支持动态内存分配，在Rust语言中需实现Trait GlobalAlloc,将该类实例化，并使用语义项#[global_allocator]进行标记。为实现Trait GlobalAlloc，需支持两个函数：
`unsafe fn alloc(&self,layout:Layout)->*mut u8;'
'unsafe fn dealloc(&self,ptr: *mut u8,layout: Layout)`
其中Layout有两个字段：size表示要分配的字节数，align表示分配的虚拟地址的最小对齐要求，即分配的地址要求是align的倍数。align必须为2的幂次。

连续内存分配算法：若直接分配到最小地址去，能保证内存连续。但回收时，若回收中间一块时，则不能扩展中间那块。这种可用内存称为外碎片。
buddy system算法：每次分配时都恰好分配一块大小是2的幂次的内存，且保证内存的开头地址需要对齐，内存开头地址是这块内存大小的倍数。