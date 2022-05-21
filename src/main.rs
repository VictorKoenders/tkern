#![no_std]
#![no_main]
#![feature(lang_items, asm_const, panic_info_message)]
#![warn(unsafe_op_in_unsafe_fn, clippy::pedantic)]

pub mod config {
    //! Config variables.
    //! 
    //! These will be generated based on your `BOARD` config. See `build.rs` for more info.
    include!(concat!(env!("OUT_DIR"), "/config.rs"));
}

// This will include the core documentation into our docs
#[doc(inline)]
pub use core;

mod dst;

use core::arch::global_asm;
use core::fmt::Write as _;
use cortex_a::registers::MPIDR_EL1;
use tock_registers::interfaces::Readable;
use utils::const_non_null::ConstNonNull;

#[cfg(not(test))]
use core::panic::PanicInfo;

// Assembly counterpart to this file.
global_asm!( include_str!("boot.s"),);

// This is called from `src/boot.s`. See that file for more info
#[no_mangle]
pub extern "C" fn _start_rust(
    dtb_addr: u32, // x0: 32 bit pointer to DTB in memory (primary core only) / 0 (secondary cores)
    _x1: u32, // always 0 for now
    _x2: u32, // always 0 for now
    _x3: u32, // always 0 for now
    start_address: u32, // x4: The start address on which the kernel started. This will be the `_start` label in our `boot.s`
) -> ! {
    let core = MPIDR_EL1.get() & 0b11;
    let mut output = QemuOutput;
    let _ = writeln!(&mut output, "Hello Rust Kernel world!");
    let _ = writeln!(&mut output, "Core {}", core);
    if let Some(dtb_addr) = ConstNonNull::new(dtb_addr as *const u8) {
        let _ = writeln!(&mut output, "DTB address: {:p}", dtb_addr);
        for offset in 0..4 {
            let _ = write!(&mut output, "0x{:02X} ", unsafe { dtb_addr.offset(offset).as_ref() });
        }

        for i in (0..config::MAX_MEMORY).step_by(4) {
            if i % 1_000_000 == 0 {
                let _ = writeln!(&mut output, "{} / {}...", i, config::MAX_MEMORY);
            }
            let slice = unsafe { &*((i + 4) as *const [u8; 4])};
            if u32::from_be_bytes(*slice) == 0xd00dfeed {
                let _ = writeln!(&mut output, "Found 0xd00dfeed (1) at 0x{:04X}", i);
                let slice = unsafe { &*((i + 4) as *const [u8; 4])};
                let _ = writeln!(&mut output, "Size {:?}, be: {}", slice, u32::from_be_bytes(*slice));
            }
        }
    }

    let addr = match core {
        0 => Some(0xE0),
        1 => Some(0xE8),
        2 => Some(0xF0),
        _ => None
    };
    if let Some(addr) = addr {
        let _ = writeln!(&mut output, "Spawning up the next core!");
        let _ = writeln!(&mut output, "--------------------------");
        unsafe {
            core::ptr::write_volatile(addr as *mut usize, start_address as usize);
        }
    }

    loop {}
}

struct QemuOutput;

impl core::fmt::Write for QemuOutput {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            unsafe { core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8); }
        }
        Ok(())
    }
}

#[cfg(not(test))]
#[lang = "eh_personality"]
pub extern fn eh_personality() {}

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
