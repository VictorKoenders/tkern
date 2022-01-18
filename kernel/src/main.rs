#![feature(format_args_nl, panic_info_message)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64/boot.rs"]
mod arch_boot;
mod bsp;
mod console;

pub fn kernel_main() -> ! {
    println!("[0] Hello from Rust!");
    panic!("Stopping here");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(args) = info.message() {
        println!("\nKernel panic: {}", args);
    } else {
        println!("\nKernel panic!");
    }
    loop {
        cortex_a::asm::wfi();
    }
}
