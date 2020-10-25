//! System module
//!
//! This module serves as an abstraction of the hardware.
//! The goal is to have a single interface where the underlying implementations do not matter.
//!
//! e.g. instead of trying to figure out what system the hardware uses to store information on disk,
//! this module allows you to request `system.fs().read_exact(idx, offset, &mut buffer)`

pub mod atapi;
pub mod pci;

mod descriptor;

use self::descriptor::Descriptor;
use crate::PhysicalAddress;

/// Reference to the hardware system that the kernel is running on
pub struct System {
    #[allow(dead_code)]
    descriptor: Descriptor,
}

impl Drop for System {
    fn drop(&mut self) {
        panic!("System should never be dropped");
    }
}

impl System {
    /// Create a new instance of the hardware system
    ///
    /// # Safety
    ///
    /// - The system must ensure that this gets only called once
    /// - The `descriptor_address` must be a valid RSDT address
    pub unsafe fn new(descriptor_address: PhysicalAddress) -> Self {
        let descriptor =
            Descriptor::scan(descriptor_address).expect("Could not scan descriptor tables");

        System { descriptor }
    }

    /// Run tests for debugging purposes
    pub fn test(&self) {
        vga_println!("For looking up these devices, go to   https://pci-ids.ucw.cz/read/PC/");
        let mut atapi_io = None;
        for device in self::pci::scan() {
            use self::pci::{BaseAddress, DeviceKind};

            vga_print!(
                "[{}:{}:{} {}] ",
                device.location.bus(),
                device.location.device(),
                device.location.function(),
                device.id.class
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

        self::atapi::test(atapi_io);
    }

    /// Get a reference to the persistent storage of this system
    pub fn storage(&self) -> SystemStorage {
        SystemStorage { system: self }
    }
}

/// Abstraction of the persistent storage available on this system.
/// This will most likely be the hard drive that is present in the device.
pub struct SystemStorage<'a> {
    #[allow(dead_code)]
    system: &'a System,
}

#[allow(unused_variables)]
impl SystemStorage<'_> {
    /// Return the amount of drives that are available on this system.
    /// This value may be cached for performance reasons.
    ///
    /// This may report more or less drives than there are storage devices in the system.
    /// Drivers are allowed to merge or split up devices based on what the driver deems best.
    ///
    /// Even though this function is marked async, some implementations may infact be executed in a synchronous matter.
    pub async fn drive_count(&self) -> Result<u8, ()> {
        Ok(0)
    }

    /// Return the storage size available on the given drive.
    /// This value may be cached for performance reasons
    ///
    /// Even though this function is marked async, some implementations may infact be executed in a synchronous matter.
    pub async fn storage_size(&self, index: u8) -> Result<usize, ()> {
        Err(())
    }

    /// Read N bytes from the given disk `index` at the given `offset`.
    /// If the buffer could not be filled, an error is thrown.
    /// The contents of `buffer` can be partially overwritten before an error is thrown.
    ///
    /// Even though this function is marked async, some implementations may infact be executed in a synchronous matter.
    pub async fn read_exact(&self, index: u8, offset: usize, buffer: &mut [u8]) -> Result<(), ()> {
        Err(())
    }

    /// Write N bytes to the given disk `index` at the given `offset`.
    /// If the full buffer could not be written, an error is thrown.
    /// It is possible for the write action to partially succeed, while still throwing an error.
    ///
    /// Even though this function is marked async, some implementations may infact be executed in a synchronous matter.
    pub async fn write_exact(&self, index: u8, offset: usize, buffer: &[u8]) -> Result<(), ()> {
        Err(())
    }
}
