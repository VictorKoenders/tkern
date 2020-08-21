// #![allow(dead_code, unused_variables)]
#![feature(lang_items, alloc_error_handler, llvm_asm)]
#![no_std]

extern crate alloc;

#[macro_use]
pub mod vga;
pub mod allocator;
pub mod arch;
pub mod memory;
pub mod memory_new;

use crate::memory_new::{AddressAccess, AllocateOptions, Mapper, PhysicalAddress};
use alloc::boxed::Box;
use core::panic::PanicInfo;

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

    /*
    if let Some(rsdp) = boot_info.rsdp_v2_tag() {
        vga_println!("Found rsdp v2");
        vga_println!("oem: {:?} rev {}", rsdp.oem_id(), rsdp.revision());
        vga_println!("xsdt address: 0x{:x}", rsdp.xsdt_address());
    } else if let Some(rsdp) = boot_info.rsdp_v1_tag() {
        vga_println!("Found rsdp v1");
        vga_println!("oem: {:?} rev {}", rsdp.oem_id(), rsdp.revision());
        vga_println!("rsdt address: 0x{:x}", rsdp.rsdt_address());
    } else {
        vga_println!("Did not find rsdp");
    }

    let mut allocator = AreaFrameAllocator::new(&boot_info);

    memory::test_paging(&mut allocator);
    */

    unsafe {
        allocator::init(&boot_info);
    }
    vga_println!("Allocating a box...");
    let b = Box::new(5);
    vga_println!("Allocated at {:p}", b);
    vga_println!("Testing deallocate...");
    drop(b);
    vga_println!("Success!");

    crate::arch::halt_loop();
    unsafe {
        memory_new::init();
    }
    let vga_addr = Mapper::access_mut(|mapper| {
        mapper.map_physical_address(PhysicalAddress(0xb8000), AllocateOptions::empty())
    });

    let mut addr = vga_addr.get_offset(4).unwrap();
    // Prints "OKAY"
    unsafe {
        addr.write(0x2f592f412f4b2f4f);
    }
    panic!("End of kernel reached");
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
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
