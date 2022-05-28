#![cfg_attr(target_arch = "aarch64", no_std)]
#![no_main]
#![feature(lang_items, asm_const, panic_info_message, strict_provenance)]
#![warn(unsafe_op_in_unsafe_fn, clippy::pedantic)]

mod hardware;
mod sys;

// This will include the core documentation into our docs
#[doc(inline)]
pub use core;

use bcm2837_hal::videocore::Color;
use core::arch::global_asm;
use core::fmt::Write as _;
use core::num::NonZeroUsize;
use core::ptr::NonNull;
use cortex_a::registers::MPIDR_EL1;
use tock_registers::interfaces::Readable;

// Assembly counterpart to this file.
global_asm!(include_str!("boot.s"));

// This is called from `src/boot.s`. See that file for more info
#[no_mangle]
pub extern "C" fn _start_rust(
    atag_addr: u64, // x0: 32 bit pointer to atag in memory (primary core only) / 0 (secondary cores)
    _x1: u64,       // always 0 for now
    _x2: u64,       // always 0 for now
    _x3: u64,       // always 0 for now
    _start_address: u64, // x4: The start address on which the kernel started. This will be the `_start` label in our `boot.s`
) -> ! {
    let core = (MPIDR_EL1.get() & 0xFF) as u8;
    let mut output = QemuOutput;
    let _ = writeln!(&mut output, "Hello Rust Kernel world!");
    let _ = writeln!(&mut output, "Core {}", core);
    let _ = writeln!(&mut output, "atag_addr 0x{:08X}", atag_addr);

    if let Some(ptr) = NonNull::new(atag_addr as *mut ()) {
        let atag = unsafe { atags::Atags::new(ptr) };
        let _ = writeln!(&mut output, "Atag:");
        for tag in atag.iter() {
            let _ = writeln!(&mut output, "  {:?}", tag);
        }
    }

    let _ = writeln!(
        &mut output,
        "Kernel is between 0x{:08X} and 0x{:08X} (size {})",
        sys::kernel_start(),
        sys::kernel_end(),
        utils::HumanReadableSize::new(sys::kernel_end() - sys::kernel_start())
    );

    let hardware = hardware::detect();
    let _ = writeln!(&mut output, "{:#?}", hardware);

    // let peripherals = unsafe { bcm2837_hal::pac::Peripherals::steal() };
    let mut videocore = unsafe {
        bcm2837_hal::videocore::VideoCore::new(
            hardware
                .mmio_base_address
                .map_addr(|a| NonZeroUsize::new_unchecked(a.get() + 0xb880))
                .cast()
                .as_ref(),
        )
    };

    let framebuffer = match videocore.framebuffer_init(800, 600) {
        Ok(fb) => fb,
        Err(e) => {
            let _ = writeln!(&mut output, "Could not initialize frame buffer: {:?}", e);
            let _ = writeln!(&mut output, "Aborting kernel");
            loop {
                cortex_a::asm::wfi();
            }
        }
    };
    let _ = writeln!(&mut output, "Frame buffer: {:#?}", framebuffer);

    for x in 10..20 {
        for y in 10..20 {
            framebuffer.put_pixel(x, y, Color::WHITE);
        }
    }

    framebuffer.text(10, 50, "Hello world!");
    framebuffer.text(10, 60, "This is Trangar's rusty kernel");

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

#[cfg(not(any(test, target_os = "linux")))]
mod rust_internals {
    use crate::QemuOutput;
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
