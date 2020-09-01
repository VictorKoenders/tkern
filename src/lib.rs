//! TKern
//!
//! An experimental kernel

#![feature(lang_items, alloc_error_handler, llvm_asm, abi_x86_interrupt)]
#![no_std]
#![warn(missing_docs)]

extern crate alloc;

#[macro_use]
pub mod vga;
pub mod allocator;
pub mod arch;
pub mod dev_utils;
pub mod interrupts;
pub mod memory;
pub mod system;
pub mod utils;

use memory::PhysicalAddress;

/// Entry point of the kernel
#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) -> ! {
    vga::set_color(vga::ColorCode::new(
        vga::Color::LightGreen,
        vga::Color::Black,
    ));
    vga::clear();

    arch::interrupts::init();

    vga_println!("TKern {}", env!("CARGO_PKG_VERSION"));

    if cfg!(debug_assertions) {
        vga_println!("!Running in debug mode, kernel will be slow!");
    }

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };

    unsafe {
        allocator::init(&boot_info);
        memory::init();
    }

    let _system = if let Some(rsdp) = boot_info.rsdp_v2_tag() {
        vga_println!("RSDP V2 at {:p}", rsdp);
        unimplemented!()
    } else if let Some(rsdp) = boot_info.rsdp_v1_tag() {
        let addr = PhysicalAddress(rsdp.rsdt_address() as u64);
        vga_println!("RSDP V1 at {:?}", addr);
        unsafe { system::System::scan(addr) }
    } else {
        panic!("Could not find rsdp, aborting");
    };

    arch::interrupts::enable();

    vga_println!("For looking up these devices, go to   https://pci-ids.ucw.cz/read/PC/");
    for device in system::pci::scan() {
        use system::pci::{BaseAddress, DeviceKind};

        vga_print!(
            "[{}:{}:{}] ",
            device.location.bus(),
            device.location.device(),
            device.location.function()
        );
        vga_print!("{:04X}/{:04X}", device.id.vendor, device.id.device);

        #[allow(irrefutable_let_patterns)]
        if let DeviceKind::General(kind) = device.kind {
            vga_print!("/{:04X}{:04X}", kind.subsystem_id, kind.subsystem_vendor_id);
            if let Some(known_name) = kind.get_known_name(&device.id) {
                vga_println!(": {}", known_name);
            } else {
                vga_println!();
            }

            let mut any_address = false;
            for (index, bar) in kind.bars.iter().filter(|b| !b.is_null()).enumerate() {
                if index == 0 {
                    vga_print!("Base address: ");
                    any_address = true;
                } else {
                    vga_print!(", ");
                }
                match bar {
                    BaseAddress::Io { address } => vga_print!("Io(0x{:X})", address),
                    BaseAddress::Memory { base_address, .. } => {
                        vga_print!("Memory(0x{:X})", base_address)
                    }
                }
            }
            if any_address {
                vga_println!();
            }
        } else {
            vga_println!();
        }
    }

    panic!("End of kernel reached");
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
