use crate::consts::MAX_PHYSICAL_PAGES;
use spin::Mutex;

pub struct SegmentTreeAllocator {
    a: [u8; MAX_PHYSICAL_PAGES << 1],//数组，大小0x8000000>>11
    m: usize,
    n: usize,
    offset: usize
}

impl SegmentTreeAllocator {
    //使用物理页号区间[l,r)进行初始化
    pub fn init(&mut self, l: usize, r: usize) {
        self.offset = l - 1;
        self.n = r - l;
        self.m = 1;
        while self.m < self.n + 2 {
            self.m = self.m << 1;
        }
        for i in (1..(self.m << 1)) { self.a[i] = 1; }
        for i in (1..self.n) { self.a[self.m + i] = 0; }
        for i in (1..self.m).rev() { self.a[i] = self.a[i << 1] & self.a[(i << 1) | 1]; }
    }

    pub fn alloc(&mut self) -> usize {
        //分配一个物理页，自上而下寻找可用的最小物理页号。假定永远不会出现物理页耗尽的情况
        if self.a[1] == 1 {
            panic!("physical memory depleted!");
        }
        let mut p = 1;
        while p < self.m {
            if self.a[p << 1] == 0 { p = p << 1; } else { p = (p << 1) | 1; }
        }
        let result = p + self.offset - self.m;
        self.a[p] = 1;
        p >>= 1;
        while p > 0 {
            self.a[p] = self.a[p << 1] & self.a[(p << 1) | 1];
            p >>= 1;
        }
        result
    }

    pub fn dealloc(&mut self, n: usize) {
        //回收物理页号为n的物理页，自下而上进行更新
        let mut p = n + self.m - self.offset;
        assert!(self.a[p] == 1);
        self.a[p] = 0;
        p >>= 1;
        while p > 0 {
            self.a[p] = self.a[p << 1] & self.a[(p << 1) | 1];
            p >>= 1;
        }
    }
}
//将类实例化并声明为static，因为在整个程序运行过程中均有效。用spin::Mutex加锁
pub static SEGMENT_TREE_ALLOCATOR: Mutex<SegmentTreeAllocator> = Mutex::new(SegmentTreeAllocator {
    a: [0; MAX_PHYSICAL_PAGES << 1],
    m: 0,
    n: 0,
    offset: 0
});