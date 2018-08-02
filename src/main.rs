#![feature(panic_implementation)] // define our own panic handler
#![feature(exclusive_range_pattern)] //used in vga_buffer.rs
#![no_std]
#![cfg_attr(not(test), no_main)] // disable rust-level compilation entry points
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

// add the library as a dependency (same crate name as executable)
#[macro_use]
extern crate phil_os;


use core::panic::PanicInfo;


// C runtime zero (crt0) and start
// are overwritten and our own
// _start is written for a new
// linker entrypoint
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! { // "!" is of the "never" type
                                  // because it never returns

    println!("Hello World{}", "!");
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
