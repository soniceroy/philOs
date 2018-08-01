#![feature(panic_implementation)]
#![feature(exclusive_range_pattern)] //used in vga_buffer.rs
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

use core::panic::PanicInfo;
extern crate bootloader_precompiled;
extern crate volatile; // avoids erroneous optimization
                       // due to compiler not knowing
                       // the buffer struct has valid
                       // side effects of writing to screen.
#[macro_use]
extern crate lazy_static;

extern crate spin;
extern crate uart_16550;

#[macro_use]
mod vga_buffer;

#[macro_use]
mod serial; // Qemu integration tests using a virtual serial uart

// C runtime zero (crt0) and start
// are overwritten and our own
// _start is written for a new
// entrypoint

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! { // "!" is of the "never" type
                                  // because it never returns

    println!("Hello World{}", "!");
    serial_println!("Hello World{}", "!");
    println!("And I'm outta here...");
    serial_println!("And I'm outta here...");
    panic!("Zippy canoe!!!");
    loop {}
}

/// This function is called on panic
#[cfg(not(test))] // only compile when the test flag is not set
#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
extern crate std;

#[cfg(test)]
extern crate array_init;
