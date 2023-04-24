The project is based on [BlogOS](https://github.com/phil-opp/blog_os).
BlogOS is a simple kernel implemented in Rust. The kernel is compiled into bootimage and then runs on QEMU simulator. The kernel is able to print to the screen using VGA text buffer, handle errors with interrupt handlers, create heaps and implement async and implement async and await support to the kernel by creating a basic executor. In order to help debugging the code, I also include testing framework.

### Repository structure
I list the main structure of the repo.
```
├── Cargo.lock
├── Cargo.toml							// Lists the main dependencies of the repo
├── README.md
├── src
│   ├── allocator						// Provides several implementation for heap allocators
│   │   ├── bump.rs						
│   │   ├── fixed_size_block.rs
│   │   └── linked_list.rs
│   ├── allocator.rs
│   ├── gdt.rs							// Creates GlobalDescriptorTable for different segments
│   ├── interrupts.rs					// Specifies the interrupt handlers
│   ├── lib.rs
│   ├── main.rs
│   ├── memory.rs						// Specifies memory mapping logistics
│   ├── serial.rs						// Print text to the screen
│   ├── task							// Creates task executor
│   │   ├── executor.rs
│   │   ├── keyboard.rs
│   │   ├── mod.rs
│   │   └── simple_executor.rs
│   └── vga_buffer.rs					// Provides VGA support
└── tests								// Tests for different components
    ├── basic_boot.rs
    ├── heap_allocation.rs
    ├── should_panic.rs
    └── stack_overflow.rs
```

### Demo
link: https://drive.google.com/file/d/13ZciTeWxxiS8cjkgHtXgb9WW-AiPebj4/view?usp=sharing


### Execution environment
1. Install Rust nightly
I use many experiemental features in Rust, so nightly version is needed to run the codes.
2. Install QEMU
The kernal runs on QEMU simulator.

### Setup
1. clone the repo with `git clone git@github.com:isabella0428/RustOS.git`
2. compile and run the code `cargo run`
