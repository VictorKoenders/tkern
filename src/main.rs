#![cfg_attr(target_arch = "aarch64", no_std)]
#![no_main]
#![feature(lang_items, asm_const, panic_info_message, strict_provenance)]
#![warn(unsafe_op_in_unsafe_fn, clippy::pedantic)]

pub mod config {
    #![allow(clippy::unreadable_literal)]

    //! Config variables.
    //!
    //! These will be generated based on your `BOARD` config. See `build.rs` for more info.
    include!(concat!(env!("OUT_DIR"), "/config.rs"));
}

// This will include the core documentation into our docs
#[doc(inline)]
pub use core;

mod dst;
mod sys;

use core::arch::global_asm;
use core::fmt::Write as _;
use cortex_a::registers::MPIDR_EL1;
use tock_registers::interfaces::Readable;

#[cfg(not(test))]
use core::panic::PanicInfo;

// Assembly counterpart to this file.
global_asm!(include_str!("boot.s"));

// This is called from `src/boot.s`. See that file for more info
#[no_mangle]
pub extern "C" fn _start_rust(
    dtb_addr: u64, // x0: 32 bit pointer to DTB in memory (primary core only) / 0 (secondary cores)
    _x1: u64,      // always 0 for now
    _x2: u64,      // always 0 for now
    _x3: u64,      // always 0 for now
    _start_address: u64, // x4: The start address on which the kernel started. This will be the `_start` label in our `boot.s`
) -> ! {
    let core = MPIDR_EL1.get() & 0b11;
    let mut output = QemuOutput;
    let _ = writeln!(&mut output, "Hello Rust Kernel world!");
    let _ = writeln!(&mut output, "Core {}", core);
    let _ = writeln!(&mut output, "dtb_addr 0x{:08X}", dtb_addr);

    let _ = writeln!(
        &mut output,
        "Kernel is between 0x{:08X} and 0x{:08X} (size {})",
        sys::kernel_start(),
        sys::kernel_end(),
        utils::HumanReadableSize::new(sys::kernel_end() - sys::kernel_start())
    );

    if core == 0 {
        dst::find_magic_header(&mut output);
    }

    // let addr = match core {
    //     0 => Some(0xE0),
    //     1 => Some(0xE8),
    //     2 => Some(0xF0),
    //     _ => None
    // };
    // if let Some(addr) = addr {
    //     let _ = writeln!(&mut output, "Spawning up the next core!");
    //     let _ = writeln!(&mut output, "--------------------------");
    //     unsafe {
    //         core::ptr::write_volatile(addr as *mut usize, start_address as usize);
    //     }
    // }

    loop {
        cortex_a::asm::wfe();
    }
}

struct QemuOutput;

impl core::fmt::Write for QemuOutput {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            unsafe {
                core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
            }
        }
        Ok(())
    }
}

#[cfg(not(test))]
#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let (location, line, column) = match info.location() {
        Some(loc) => (loc.file(), loc.line(), loc.column()),
        _ => ("???", 0, 0),
    };

    let _ = writeln!(
        &mut QemuOutput,
        "\nPanic: {}\
         \n       {}:{}:{}\n",
        info.message().unwrap_or(&format_args!("explicit panic")),
        location,
        line,
        column
    );
    loop {}
}
