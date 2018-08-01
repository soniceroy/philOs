#![feature(panic_implementation)]
#![feature(exclusive_range_pattern)] //used in vga_buffer.rs
#![no_std]
#![no_main]

use core::panic::PanicInfo;
extern crate bootloader_precompiled;
extern crate volatile; // avoids erroneous optimization
                       // due to compiler not knowing
                       // the buffer struct has valid
                       // side effects of writing to screen.
#[macro_use]
extern crate lazy_static;
extern crate spin;

#[macro_use]
mod vga_buffer;

// C runtime zero (crt0) and start
// are overwritten and our own
// _start is written for a new
// entrypoint

#[no_mangle]
pub extern "C" fn _start() -> ! { // "!" is of the "never" type
                                  // because it never returns

    println!("Hello World{}", "!");
    println!("And I'm outta here...");
    panic!("Zippy canoe!!!");
    loop {}
}

/// This function is called on panic
#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
