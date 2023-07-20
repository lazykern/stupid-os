#![no_std] // disable standard library
#![no_main] // disable main

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // disable name mangling
pub extern "C" fn _start() -> ! {
    loop {}
}

