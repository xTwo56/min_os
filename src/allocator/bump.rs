use super::{Locked, align_up};
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr;

/// A linear allocator that advances `next` for every allocation.
///
/// It is extremely fast, but it cannot reclaim individual holes. The entire
/// heap becomes reusable only after every outstanding allocation is freed.
pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    /// Create an empty allocator. `init` must be called before it is used.
    pub const fn new() -> Self {
        BumpAllocator {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }

    /// Initialize the allocator with an unused heap region.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the memory range is valid and unused, and
    /// that this function is called only once for this allocator.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start.saturating_add(heap_size);
        self.next = heap_start;
    }
}

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut bump = self.lock();

        // Padding may be required so the returned pointer satisfies Layout.
        let alloc_start = align_up(bump.next, layout.align());
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return ptr::null_mut(),
        };

        if alloc_end > bump.heap_end {
            ptr::null_mut()
        } else {
            bump.next = alloc_end;
            bump.allocations += 1;
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut bump = self.lock();

        bump.allocations -= 1;
        // A bump allocator cannot reuse one freed object. Once all objects are
        // gone, resetting `next` makes the complete heap available again.
        if bump.allocations == 0 {
            bump.next = bump.heap_start;
        }
    }
}
