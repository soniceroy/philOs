#![feature(panic_implementation)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

// C runtime zero (crt0) and start
// are overwritten and our own
// _start is written for a new
// entrypoint
#[no_mangle]
pub extern "C" fn _start() -> ! { // never type "!"
    loop {}                       // because it never returns
}

/// This function is called on panic
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
