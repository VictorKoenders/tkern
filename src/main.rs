#![cfg_attr(target_arch = "aarch64", no_std)]
#![no_main]
#![feature(
    lang_items,
    asm_const,
    panic_info_message,
    strict_provenance,
    allocator_api,
    nonnull_slice_from_raw_parts,
    alloc_error_handler,
    format_args_nl
)]
#![warn(unsafe_op_in_unsafe_fn, clippy::pedantic, rust_2018_idioms)]
#![allow(clippy::cast_possible_truncation)]

extern crate alloc;

#[macro_use]
mod macros;

mod allocator;
mod hardware;
mod output;
mod sys;

// This will include the core documentation into our docs
#[doc(inline)]
pub use core;

use atags::AtagMemory;
use core::arch::global_asm;
use core::num::NonZeroUsize;
use core::ptr::NonNull;
use videocore_mailbox::VideoCore;

// Assembly counterpart to this file.
global_asm!(include_str!("boot.s"));

/// This is called from `src/boot.s`. See that file for more info
///
/// # Panics
///
/// Will panic if the memory size could not be detected, or if it was 0
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
        let mut videocore = unsafe { VideoCore::new(hardware.mmio_base_address) };

        let framebuffer = videocore.allocate_framebuffer(800, 600, 24);
        crate::output::framebuffer::init(framebuffer);

        println!("Hello Rust Kernel world!");

        let mut memory: Option<AtagMemory> = None;
        if let Some(ptr) = NonNull::new(atag_addr as *mut ()) {
            let atag = unsafe { atags::Atags::new(ptr) };
            println!("Atag:");
            for tag in atag.iter() {
                print!("  {:?}", tag);
                match tag {
                    atags::Atag::Memory(mem) => {
                        memory = Some(mem.clone());
                        println!();
                    }
                    _ => {
                        println!(" (ignored)");
                    }
                }
            }
        }
        println!(
            "Kernel is between 0x{:08X} and 0x{:08X} (size {})",
            sys::kernel_start(),
            sys::kernel_end(),
            utils::HumanReadableSize::new(sys::kernel_end() - sys::kernel_start())
        );
        println!("{:#?}", hardware);

        let memory = memory.expect("Memory size not detected");
        let mut memory_start = (memory.start as usize).max(sys::kernel_end());
        if memory_start % 16 != 0 {
            memory_start += 16 - (memory_start % 16);
        }
        let memory_length = memory.size as usize + memory.start as usize - memory_start;
        println!(
            "Memory starts at 0x{:08X} and is {:?} (end = 0x{:08X})",
            memory_start,
            utils::HumanReadableSize::new(memory_length as usize),
            memory.start + memory.size
        );
        let length = NonZeroUsize::new(memory_length).expect("Memory size is detected to be 0");
        unsafe {
            allocator::init(NonNull::new_unchecked(memory_start as *mut ()), length);
        }
        let box_ = alloc::boxed::Box::new(5);
        println!("Box allocated at {:p}", box_);
        drop(box_);

        unsafe {
            hardware.spawn_other_cores(NonNull::new_unchecked(start_address as usize as *mut _));
        }
    }

    loop {
        cortex_a::asm::wfe();
    }
}

#[cfg(not(any(test, target_os = "linux")))]
mod rust_internals {
    use core::panic::PanicInfo;

    #[lang = "eh_personality"]
    pub extern "C" fn eh_personality() {}

    #[panic_handler]
    fn panic(info: &'_ PanicInfo<'_>) -> ! {
        let (location, line, column) = match info.location() {
            Some(loc) => (loc.file(), loc.line(), loc.column()),
            _ => ("???", 0, 0),
        };

        println!(
            "\nPanic: {}\
            \n       {}:{}:{}\n",
            info.message().unwrap_or(&format_args!("explicit panic")),
            location,
            line,
            column
        );
        loop {}
    }

    #[alloc_error_handler]
    fn oom(layout: core::alloc::Layout) -> ! {
        panic!("Could not allocate {:?}", layout);
    }
}
