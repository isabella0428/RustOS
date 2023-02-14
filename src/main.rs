// src/main.rs

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use blog_os::memory;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

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

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use blog_os::memory::BootInfoFrameAllocator;
    use blog_os::memory::{active_4_table, translate_addr};
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Hello World{}", "!");
    blog_os::init();

    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }

    // trigger a page fault
    //stack_overflow();

    // use x86_64::registers::control::Cr3;

    // let (level_4_page_table, _) = Cr3::read();
    // println!(
    //     "Level 4 page table at: {:?}",
    //     level_4_page_table.start_address()
    // );

    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    // let addresses = [
    //     0xb8000,
    //     0x201008,
    //     0x0100_0020_1a10,
    //     boot_info.physical_memory_offset,
    // ];

    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     let phys = unsafe { translate_addr(virt, phys_mem_offset) };
    //     println!("{:?}->{:?}", virt, phys);
    // }

    // let l4_table = unsafe { active_4_table(phys_mem_offset) };

    // for (i, entry) in l4_table.iter().enumerate() {
    //     use x86_64::structures::paging::PageTable;

    //     if !entry.is_unused() {
    //         println!("L4 Entry {}: {:?}", i, entry);

    //         let phys = entry.frame().unwrap().start_address();
    //         let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //         let ptr = VirtAddr::new(virt).as_mut_ptr();
    //         let l3_table: &PageTable = unsafe { &*ptr };

    //         for (i, entry) in l3_table.iter().enumerate() {
    //             if !entry.is_unused() {
    //                 println!("  L3 Entry {}: {:?}", i, entry);
    //             }
    //         }
    //     }
    // }

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    blog_os::hlt_loop();
}
