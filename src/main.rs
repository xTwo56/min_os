#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(min_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    min_os::println!("shoutout to sangeet{}", "!");

    #[cfg(test)]
    test_main();

    init();

    // x86_64::instructions::interrupts::int3();
    //
    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // };

    loop {}
}

fn init() {
    interrupt::init_idt();
}

#[test_case]
fn trivial_assertion() {
    min_os::serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    min_os::serial_println!("[ok]");
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    min_os::println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    min_os::test_panic_handler(info)
}
