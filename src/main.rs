#![no_std]
#![no_main]

pub mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"shoutout to sangeet";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("shoutout to sangeet{}", "!");

    loop {}
}
