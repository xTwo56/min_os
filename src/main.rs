#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(min_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

// Heap-backed collections live in alloc, which is usable without std.
extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use min_os::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use min_os::allocator;
    use min_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    println!("shoutout to sangeet{}", "!");
    min_os::init();

    // Reuse post 09's page-table access and unused-frame allocator. Heap setup
    // replaces the earlier VGA mapping demo and uses real unused RAM frames.
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // Must precede Box::new, Vec growth, or Rc::new: the allocator starts empty.
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // Box owns a heap allocation; {:p} prints the address of the stored value.
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // Vec keeps its elements on the heap and reallocates as capacity grows.
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // Rc::clone shares the allocation rather than copying the vector. The
    // strong count becomes 2, then drops to 1 when the original owner is dropped.
    // The allocation is freed when the last owner is dropped.
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    min_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    min_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    min_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
