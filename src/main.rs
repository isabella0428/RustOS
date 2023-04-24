// src/main.rs

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

mod serial;
mod vga_buffer;

use alloc::{boxed::Box};
use blog_os::task::{executor::Executor, Task};
use blog_os::{memory, task::keyboard};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

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

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

fn print_to_screen_example() {
    // VGA Text buffer
    println!("Hello World{}", "!");
}

fn memory_mapping_example(boot_info: &'static BootInfo) {
    use x86_64::{VirtAddr};
    use blog_os::memory::active_level_4_table;
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }
}

fn heap_example(boot_info: &'static BootInfo) {
    use blog_os::allocator;
    use x86_64::{ VirtAddr};
    use blog_os::memory::BootInfoFrameAllocator;

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    let heap_value = Box::new(41);
    println!("heap_value at {:p} {}", heap_value, heap_value);
}

fn executor_example() {
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    blog_os::init();

    print_to_screen_example();
    memory_mapping_example(boot_info);
    heap_example(boot_info);
    executor_example();

    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    blog_os::hlt_loop();
}
