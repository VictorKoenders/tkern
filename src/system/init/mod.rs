pub mod pci;

use super::{System};

/// Create a new instance of the hardware system
pub unsafe fn init(_system: &mut System) {
    vga_println!("For looking up these devices, go to   https://pci-ids.ucw.cz/read/PC/");
    for device in self::pci::scan() {
        use self::pci::{BaseAddress, DeviceKind};

        vga_print!(
            "[{}:{}:{} {:?}] ",
            device.location.bus(),
            device.location.device(),
            device.location.function(),
            device.id.class,
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
                    BaseAddress::Io { address } => {
                        vga_print!("Io(0x{:X})", address);
                    }
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
}
