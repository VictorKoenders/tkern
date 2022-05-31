#![cfg_attr(target_arch = "aarch64", no_std)]
#![no_main]
#![feature(lang_items, asm_const, panic_info_message, strict_provenance)]
#![warn(unsafe_op_in_unsafe_fn, clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]

mod hardware;
mod output;
mod sys;

// This will include the core documentation into our docs
#[doc(inline)]
pub use core;

use core::arch::global_asm;
use core::fmt::Write as _;
use core::ptr::NonNull;
use videocore_mailbox::VideoCore;

// Assembly counterpart to this file.
global_asm!(include_str!("boot.s"));

// This is called from `src/boot.s`. See that file for more info
#[no_mangle]
pub extern "C" fn _start_rust(
    atag_addr: u64, // x0: 32 bit pointer to atag in memory (primary core only) / 0 (secondary cores)
    _x1: u64,       // always 0 for now
    _x2: u64,       // always 0 for now
    _x3: u64,       // always 0 for now
    start_address: u64, // x4: The start address on which the kernel started. This will be the `_start` label in our `boot.s`
) -> ! {
    let hardware = hardware::detect();
    if hardware.is_primary_core() {
        let mut output = output::QemuOutput;
        print_init(&mut output, atag_addr);

        let _ = writeln!(&mut output, "{:#?}", hardware);

        let mut videocore = unsafe { VideoCore::new(hardware.mmio_base_address) };

        let framebuffer = videocore.allocate_framebuffer(1600, 1200, 24);
        let mut output = output::FrameBufferOutput::new(framebuffer);

        print_init(&mut output, atag_addr);

        unsafe {
            hardware.spawn_other_cores(NonNull::new_unchecked(start_address as usize as *mut _));
        }
    }

    loop {
        cortex_a::asm::wfe();
    }
}

fn print_init(output: &mut impl core::fmt::Write, atag_addr: u64) {
    let _ = writeln!(output, "Hello Rust Kernel world!");
    let _ = writeln!(output, "atag_addr 0x{:08X}", atag_addr);
    if let Some(ptr) = NonNull::new(atag_addr as *mut ()) {
        let atag = unsafe { atags::Atags::new(ptr) };
        let _ = writeln!(output, "Atag:");
        for tag in atag.iter() {
            let _ = writeln!(output, "  {:?}", tag);
        }
    }
    let _ = writeln!(
        output,
        "Kernel is between 0x{:08X} and 0x{:08X} (size {})",
        sys::kernel_start(),
        sys::kernel_end(),
        utils::HumanReadableSize::new(sys::kernel_end() - sys::kernel_start())
    );
}

#[cfg(not(any(test, target_os = "linux")))]
mod rust_internals {
    use crate::output::QemuOutput;
    use core::fmt::Write;
    use core::panic::PanicInfo;

    #[lang = "eh_personality"]
    pub extern "C" fn eh_personality() {}

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
}
