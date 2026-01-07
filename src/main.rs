// Disable the standard library
#![no_std]
#![no_main]
use core::panic::PanicInfo;

#[panic_handler]
// ! is the never type, must never return
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}
