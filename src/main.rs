// src/main.rs

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

// // don't mangle the name of this function
// // this function is the entry point, since the linker looks for a function
// // named `_start` by default

// #[no_mangle]
// pub extern "C" fn _start() {
//     println!("Hello World{}", "!");

//     blog_os::init();

//     // Invoke a breakpoint exception
//     x86_64::instructions::interrupts::int3();

//     #[cfg(test)]
//     test_main();

//     println!("It did not crash");
//     loop {}
// }

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    blog_os::init();

    // trigger a page fault
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };

    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}