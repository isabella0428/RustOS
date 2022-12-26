// src/main.rs

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buffer;

use core::panic::PanicInfo;
// /// This function is called on panic.
// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     println!("{}", _info);
//     loop {}
// }

// don't mangle the name of this function
// this function is the entry point, since the linker looks for a function
// named `_start` by default
#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");
    loop {}
}


/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}