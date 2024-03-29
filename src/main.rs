#![cfg_attr(target_os = "tkern", no_std)]
#![cfg_attr(target_os = "tkern", no_main)]
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

#[allow(unused_extern_crates)]
extern crate alloc;

#[macro_use]
mod macros;

mod hardware;
mod output;
mod sys;
mod time;

// This will include the core documentation into our docs
#[doc(inline)]
pub use core;

use allocator::Allocator;
use atags::AtagMemory;
use core::num::NonZeroUsize;
use core::ptr::NonNull;
use utils::ReadCell;
use videocore_mailbox::VideoCore;

#[cfg(target_os = "tkern")]
use core::arch::global_asm;

// Assembly counterpart to this file.
#[cfg(target_os = "tkern")]
global_asm!(include_str!("boot.s"));

static START_ADDRESS: ReadCell<u64> = ReadCell::new(0);
static ATAG_ADDR: ReadCell<u64> = ReadCell::new(0);

/// Entry point of the kernel. Called from `sys::_start_rust`.
///
/// When this function is called, the following static variables will be initialized:
/// - [`START_ADDRESS`]
/// - [`ATAG_ADDR`]
///
/// This function will initialize:
/// - A [`videocore_mailbox::FrameBuffer`]
/// - The [memory allocator](allocator)
///
/// # Panics
///
/// Will panic if the memory size could not be detected, or if it was 0
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let hardware = hardware::detect();
    if hardware.is_primary_core() {
        let mut videocore = unsafe { VideoCore::new(hardware.mmio_base_address) };

        let framebuffer = videocore.allocate_framebuffer(1366, 768, 24);
        let (width, height) = (framebuffer.width(), framebuffer.height());
        crate::output::framebuffer::init(framebuffer);

        info!("Hello Rust Kernel world!");
        info!("Framebuffer is {}x{} pixels", width, height);

        let mut memory: Option<AtagMemory> = None;
        if let Some(ptr) = NonNull::new(*ATAG_ADDR as *mut ()) {
            let atag = unsafe { atags::Atags::new(ptr) };
            info!("Atag:");
            for tag in atag.iter() {
                output::info(|w| {
                    use core::fmt::Write as _;
                    let _ = w.write_fmt(format_args!("  {tag:?}"));
                    match tag {
                        atags::Atag::Memory(mem) => {
                            memory = Some(mem.clone());
                            let _ = w.write_char('\n');
                        }
                        _ => {
                            let _ = w.write_str(" (ignored)\n");
                        }
                    }
                });
            }
        }
        info!(
            "Kernel is between 0x{:08X} and 0x{:08X} (size {})",
            sys::kernel_start(),
            sys::kernel_end(),
            utils::HumanReadableSize::new(sys::kernel_end() - sys::kernel_start())
        );
        info!("{:#?}", hardware);
        info!("EL {:?}", sys::current_exception_level());

        let memory = memory.expect("Memory size not detected");
        let mut memory_start = (memory.start as usize).max(sys::kernel_end());
        if memory_start % 16 != 0 {
            memory_start += 16 - (memory_start % 16);
        }
        let memory_length = memory.size as usize + memory.start as usize - memory_start;
        info!(
            "Memory starts at 0x{:08X} and is {} (end = 0x{:08X})",
            memory_start,
            utils::HumanReadableSize::new(memory_length),
            memory.start + memory.size
        );
        let length = NonZeroUsize::new(memory_length).expect("Memory size is detected to be 0");
        unsafe { ALLOCATOR.init(NonNull::new_unchecked(memory_start as *mut ()), length) }
        // unsafe {
        //     hardware.spawn_other_cores(NonNull::new_unchecked(START_ADDRESS.copied() as *mut ()));
        // }
    }
    loop {
        cortex_a::asm::wfe();
    }
}

#[cfg_attr(target_os = "tkern", global_allocator)]
static mut ALLOCATOR: Allocator = unsafe { Allocator::new() };

#[cfg(target_os = "tkern")]
mod rust_internals;

// Needed for code coverage
#[cfg(not(target_os = "tkern"))]
fn main() {}
