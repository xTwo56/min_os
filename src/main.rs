#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(min_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use min_os::hlt_loop;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    min_os::println!("shoutout to sangeet{}", "!");

    min_os::init();
    // loop {
    //     min_os::print!("-"); // new
    // }
    // x86_64::instructions::interrupts::int3();

    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // };

    #[cfg(test)]
    test_main();

    hlt_loop();
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
    min_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    min_os::test_panic_handler(info);
}
