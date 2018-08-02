#![feature(panic_implementation)] // define our own panic handler
#![feature(exclusive_range_pattern)] //used in vga_buffer.rs
#![feature(abi_x86_interrupt)]
#![no_std]
#![cfg_attr(not(test), no_main)] // disable rust-level compilation entry points
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

// add the library as a dependency (same crate name as executable)
#[macro_use]
extern crate phil_os;
extern crate x86_64;
#[macro_use]
extern crate lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};

use core::panic::PanicInfo;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut ExceptionStackFrame) {

    println!("EXCEPTION: BREAKPOINT\n{:?}", stack_frame);
}
// C runtime zero (crt0) and start
// are overwritten and our own
// _start is written for a new
// linker entrypoint
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! { // "!" is of the "never" type
                                  // because it never returns

    println!("Hello World{}", "!");

    init_idt();

    // invoke a breakpoint exception
    x86_64::instructions::int3();

    println!("It did not crash!");
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
