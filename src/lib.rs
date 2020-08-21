//! TKern
//!
//! An experimental kernel

#![feature(lang_items, alloc_error_handler, llvm_asm)]
#![no_std]
#![warn(missing_docs)]

extern crate alloc;

#[macro_use]
pub mod vga;
pub mod allocator;
pub mod arch;
pub mod memory;
pub mod system;

use memory::PhysicalAddress;

/// Entry point of the kernel
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
        memory::init();
    }

    if let Some(rsdp) = boot_info.rsdp_v2_tag() {
        vga_println!("Found rsdp v2 at {:p}", rsdp);
    } else if let Some(rsdp) = boot_info.rsdp_v1_tag() {
        vga_println!("Found rsdp v1 at {:p}", rsdp);

        let mapping = system::TableAllocator::default();
        let root = unsafe { mapping.get_table(PhysicalAddress(rsdp.rsdt_address() as u64)) };

        print_table(0, root, &mapping);
    } else {
        vga_println!("Could not find rsdp");
    }

    panic!("End of kernel reached");
}

fn print_table(depth: usize, table: system::Table, allocator: &system::TableAllocator) {
    let prefix = alloc::string::String::from(' ').repeat(depth);
    vga_println!(
        "{}{:?} (payload length: {})",
        prefix,
        table.header().signature(),
        table.header().length,
    );

    match table {
        system::Table::Root(r) => {
            for child in r.entries(allocator) {
                print_table(depth + 1, child, allocator);
            }
        }
        _ => {}
    }
}

#[cfg(not(any(target_os = "linux")))]
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
