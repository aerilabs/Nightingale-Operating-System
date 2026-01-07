// Disable the standard library
#![no_std] // std library not linked
#![no_main] // disable all rust-level entry points
use core::panic::PanicInfo;

#[panic_handler]
// ! is the never type, must never return
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)] // disable name mangling
pub extern "C" fn _start() -> ! {
    // SERVES AS THE ENTRY POINT TO THE PROGRAM since the linker looks for a function named `_start` by default
    loop {}
}
