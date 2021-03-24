pub mod pci;
mod pci_storage;

use super::System;

/// Create a new instance of the hardware system
///
/// # Safety
///
/// Should be called exactly once, and
pub fn init(system: &mut System) {
    vga_println!("For looking up these devices, go to   https://pci-ids.ucw.cz/read/PC/");
    for device in self::pci::scan() {
        use self::pci::{BaseAddress, DeviceClass, DeviceKind};

        vga_print!(
            "[{}:{}:{} {:?}] ",
            device.location.bus(),
            device.location.device(),
            device.location.function(),
            device.id.class,
        );
        vga_print!("{:04X}/{:04X}", device.id.vendor, device.id.device);

        #[allow(irrefutable_let_patterns)]
        if let DeviceKind::General(kind) = &device.kind {
            vga_print!("/{:04X}{:04X}", kind.subsystem_id, kind.subsystem_vendor_id);
            if let Some(known_name) = kind.get_known_name(&device.id) {
                vga_println!(": {}", known_name);
            } else {
                vga_println!();
            }

            let mut any_address = false;
            for (index, bar) in kind.bars.iter().enumerate().filter(|(_, b)| !b.is_null()) {
                if !any_address {
                    vga_print!("Base address: ");
                    any_address = true;
                } else {
                    vga_print!(", ");
                }
                match bar {
                    BaseAddress::Io { address } => {
                        vga_print!("[{}] Io(0x{:X})", index, address);
                        if device.id.class == DeviceClass::SerialBusController {
                            system.ata_bus_position = *address as u16;
                        }
                    }
                    BaseAddress::Memory { base_address, .. } => {
                        vga_print!("[{}] Memory(0x{:X})", index, base_address)
                    }
                }
            }
            if any_address {
                vga_println!();
            }
            if kind.interrupt_pin != 0 || kind.interrupt_line != 0 {
                vga_println!("Interrupt: pin {}, line {}", kind.interrupt_pin, kind.interrupt_line);
            }
        } else {
            vga_println!();
        }


        if device.id.class == DeviceClass::MassStorageController {
            system
                .storage
                .register(pci_storage::PciStorage::new(device));
        }
    }
}
