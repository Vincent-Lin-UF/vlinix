// No Standard library to not implicitly link to the std
#![no_std]

// Overwriting the Entry point
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}

// Need to add panic handler as it is part of the standard library
// However removing std  will remove the default panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}