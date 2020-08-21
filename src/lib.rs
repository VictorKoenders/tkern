// #![allow(dead_code, unused_variables)]
#![feature(lang_items, alloc_error_handler, llvm_asm)]
#![no_std]

extern crate alloc;

#[macro_use]
pub mod vga;
pub mod allocator;
pub mod arch;
// pub mod memory;
pub mod memory_new;

use crate::memory_new::{AllocateOptions, Mapper, PhysicalAddress};

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) -> ! {
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
        memory_new::init();
    }
    let vga_addr = Mapper::access_mut(|mapper| {
        mapper.map_physical_address(
            PhysicalAddress(0xb8000),
            AllocateOptions::WRITABLE | AllocateOptions::USER_ACCESSIBLE,
        )
    });
    unsafe {
        vga::set_base_address(vga_addr.virtual_address());
    }

    vga_println!("Hello from relocated VGA!");
    panic!("End of kernel reached");
}

#[cfg(not(any(target_os = "linux")))]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    vga_println!("{}", info);
    loop {}
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
