#![feature(panic_implementation)] // required for defining the panic handler
#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

// add the library as dependency (same crate name as executable)
#[macro_use]
extern crate phil_os;

use core::panic::PanicInfo;
use phil_os::exit_qemu;

/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
#[cfg(not(test))]
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
	panic!("panic test!!!!!!!");
}


/// This function is called on panic.
#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    serial_println!("ok");

    unsafe { exit_qemu(); }
    loop {}
}