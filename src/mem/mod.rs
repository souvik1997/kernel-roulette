use core::alloc::{GlobalAlloc, Layout};

#[derive(Default)]
pub struct KernelAllocator;

impl KernelAllocator {
    pub const fn new() -> Self {
        Self {}
    }
}

// Use shim functions to avoid hardcoding the GFP_KERNEL constant
#[allow(dead_code)]
extern "C" {
    fn kmalloc_c(size: usize) -> *mut u8;
    fn kfree_c(ptr: *mut u8);
    fn krealloc_c(ptr: *mut u8, size: usize) -> *mut u8;
}

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // A side effect of the buddy allocator is that allocations are aligned to
        // the power-of-two that is larger than the allocation size. So if the
        // request needs to be aligned to something larger than the allocation size,
        // we can just pass max(size, align) to kmalloc to get something reasonable
        // at the cost of a few extra wasted bytes.
        use core::cmp::max;
        let p = kmalloc_c(max(layout.size(), layout.align()));
        if p.is_null() {
            0 as *mut u8            
        } else {
            p
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        kfree_c(ptr);
    }
}
