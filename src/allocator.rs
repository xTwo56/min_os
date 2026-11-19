// Paging supplies whole physical frames; the allocator splits the mapped heap
// into suitably sized and aligned allocations for Box, Vec, and other alloc types.

use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use linked_list_allocator::LockedHeap;
use x86_64::{
    VirtAddr,
    structures::paging::{
        FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB, mapper::MapToError,
    },
};

// This is a virtual address, not a physical RAM address. The tutorial reserves
// this unused range for the heap; its physical frames need not be contiguous.
pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

// Rust routes allocations and deallocations through this instance. LockedHeap
// protects its free list with a spinlock. It starts empty, so allocations must
// wait until init_heap finishes. Interrupt handlers must not allocate while
// interrupted code might hold this lock, or they could deadlock.
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Map the heap's pages, then make that memory available to the global allocator.
/// Call once during boot, before creating any heap-backed values.
pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        // The range is inclusive: subtract one to select the last byte inside
        // the heap instead of accidentally including an extra page.
        let heap_end = heap_start + HEAP_SIZE as u64 - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        // Give each 4 KiB virtual page its own unused physical frame. The `?`
        // propagates exhaustion or mapping errors to the boot entry point.
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        // SAFETY: the boot-info allocator supplies unused frames and this heap
        // range is reserved for these mappings. map_to may allocate additional
        // page-table frames through the same allocator. Flush the TLB entry so
        // the CPU observes the new mapping immediately.
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    // SAFETY: every byte in this region is now mapped and writable. Initialize
    // the free list only after all mappings succeed, and only once at boot.
    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}

// Retained from the article to demonstrate GlobalAlloc. This is NOT the global
// allocator: returning null signals failure, and no allocation can be freed.
pub struct Dummy;

unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should be never called")
    }
}
