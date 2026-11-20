#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(min_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

// These are the post-10 integration tests, run in their own bootable kernel.
extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use min_os::allocator::HEAP_SIZE;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use min_os::allocator;
    use min_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    // Each integration-test binary boots independently, so it must initialize
    // paging access and the heap before its test functions can allocate.
    min_os::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    test_main();
    loop {}
}

#[test_case]
fn simple_allocation() {
    // Keep two allocations alive together to check that their values coexist.
    let heap_value_1 = Box::new(41);
    let heap_value_2 = Box::new(13);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 13);
}

#[test_case]
fn large_vec() {
    // Growing a Vec exercises allocation, reallocation, and data preservation.
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_boxes() {
    // Each Box is dropped at the end of the iteration. Total allocation demand
    // exceeds HEAP_SIZE, so this only completes if freed memory can be reused.
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

#[test_case]
fn many_boxes_long_lived() {
    // Keep one allocation alive while repeatedly allocating and freeing others.
    // A bump allocator cannot reset while this value exists; the fixed-size
    // allocator succeeds because it immediately reuses each freed block.
    let long_lived = Box::new(1);
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    assert_eq!(*long_lived, 1);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    min_os::test_panic_handler(info)
}
