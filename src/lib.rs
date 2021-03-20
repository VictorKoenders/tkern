//! TKern
//!
//! An experimental kernel

#![feature(
    lang_items,
    alloc_error_handler,
    llvm_asm,
    abi_x86_interrupt,
    never_type
)]
#![no_std]
// #![warn(missing_docs)]

extern crate alloc;

#[macro_use]
pub mod vga;
#[macro_use]
pub mod utils;
pub mod allocator;
pub mod arch;
pub mod dev_utils;
pub mod futures;
pub mod interrupts;
pub mod system;

pub use self::system::memory::{PhysicalAddress, VirtualAddress};

/// Entry point of the kernel
#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) -> ! {
    crate::arch::interrupts::init();
    crate::arch::interrupts::enable();

    vga::set_color(vga::ColorCode::new(
        vga::Color::LightGreen,
        vga::Color::Black,
    ));

    vga::clear();
    vga_println!("TKern {}", env!("CARGO_PKG_VERSION"));

    if cfg!(debug_assertions) {
        vga_println!("!Running in debug mode, kernel will be slow!");
    }

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };

    unsafe {
        allocator::init(&boot_info);
    }

    /*let _system = if let Some(rsdp) = boot_info.rsdp_v2_tag() {
        vga_println!("RSDP V2 at {:p}", rsdp);
        unimplemented!()
    } else if let Some(rsdp) = boot_info.rsdp_v1_tag() {
        let addr = PhysicalAddress(rsdp.rsdt_address() as u64);
        vga_println!("RSDP V1 at {:?}", addr);
        unsafe { system::System::new(addr) }
    } else {
        panic!("Could not find rsdp, aborting");
    };*/

    crate::futures::RUNTIME.run(async move {
        loop {
            let key = crate::futures::io::Keyboard::next_key().await;
            if key.character == 'c' && key.modifiers.ctrl_pressed {
                unsafe { dev_utils::exit_qemu(dev_utils::QemuExitCode::Success) };
            }
            vga_print!("{}", key.character);
        }
    });
}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    vga_println!("{}", info);
    crate::arch::halt_loop();
}

#[derive(Debug, Copy, Clone)]
#[repr(packed)]
struct Rsdp {
    signature: [u8; 8],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_id: [u8; 6],
    oem_revision: u32,
    creator_id: u32,
    creator_revision: u32,
}
