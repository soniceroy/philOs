#![feature(panic_implementation)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
extern crate bootloader_precompiled;

// C runtime zero (crt0) and start
// are overwritten and our own
// _start is written for a new
// entrypoint
static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! { // "!" is of the "never" type
                                  // because it never returns

    // raw pointer cast to the vga buffer address 0xb8000
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // set the char in the VGA buffer to the i-ith HELLO char
            *vga_buffer.offset(i as isize * 2) = byte;
            // set the corresponding color to 0xb (light cyan)
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/// This function is called on panic
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
