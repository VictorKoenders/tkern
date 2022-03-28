#![no_std]

pub extern crate cortex_a;

pub mod boot;

mod cpu;
mod driver;

use core::{arch::asm, hint::unreachable_unchecked};

use cortex_a::registers::CurrentEL;
use tock_registers::interfaces::Readable;

/// Configure the main entry point of the kernel.
/// This should be the firstmost entry in `main.rs`, as this loads in custom asm
#[macro_export]
macro_rules! main {
    ($e:ident) => {
        core::arch::global_asm!(include_str!("../sys/aarch64/src/boot.s"));

        /// The Rust entry of the `kernel` binary.
        ///
        /// The function is called from the assembly `_start` function.
        ///
        /// # Safety
        ///
        /// - Exception return from EL2 must must continue execution in EL1 with `kernel_init()`.
        #[no_mangle]
        pub unsafe fn _start_rust(phys_boot_core_stack_end_exclusive_addr: u64) -> ! {
            $crate::boot::prepare_el2_to_el1_transition(
                phys_boot_core_stack_end_exclusive_addr,
                $e as *const () as u64,
            );

            // Use `eret` to "return" to EL1. This results in execution of kernel_init() in EL1.
            $crate::cortex_a::asm::eret();
        }
    };
}
/// Board identification.
pub fn board_name() -> &'static str {
    "Raspberry Pi 4"
}

pub fn current_privilege_level() -> &'static str {
    match CurrentEL
        .read_as_enum(CurrentEL::EL)
        .expect("Could not get EL")
    {
        CurrentEL::EL::Value::EL3 => "EL3 (Firmware)",
        CurrentEL::EL::Value::EL2 => "EL2 (Hypervisor)",
        CurrentEL::EL::Value::EL1 => "EL1 (OS Kernel)",
        CurrentEL::EL::Value::EL0 => "EL0 (Userspace)",
    }
}

pub fn infinite_loop() -> ! {
    loop {
        cortex_a::asm::wfe();
    }
}

pub fn uart() -> &'static dyn ::driver::uart::All {
    &driver::UART
}

pub fn time() -> &'static dyn ::driver::time::Time {
    if !driver::TIME_MANAGER.is_inited() {
        driver::TIME_MANAGER.init();
    }
    &driver::TIME_MANAGER
}

/// Jumps to a given function
///
/// # Safety
///
/// This function will perform a jump to an unknown address.
/// It is assumed that the given address is a valid function pointer, and that the function will never return.
pub unsafe fn jump(fun: fn() -> !) -> ! {
    let addr = fun as *const () as usize;
    asm!("BR {}", in(reg) addr);
    unreachable_unchecked();
}
