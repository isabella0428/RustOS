The project is based on [BlogOS](https://github.com/phil-opp/blog_os).
BlogOS is a simple kernel implemented in Rust. The kernel is compiled into bootimage and then runs on QEMU simulator. The kernel is able to print to the screen using VGA text buffer, handle errors with interrupt handlers, create heaps and implement async and implement async and await support to the kernel by creating a basic executor. In order to help debugging the code, I also include testing framework.

### Execution environment
1. Install Rust nightly
I use many experiemental features in Rust, so nightly version is needed to run the codes.
2. Install QEMU
The kernal runs on QEMU simulator.

### Setup
1. clone the repo with `git clone git@github.com:isabella0428/RustOS.git`
2. compile and run the code `cargo run`
