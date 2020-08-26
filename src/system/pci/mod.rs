//! PCI is a system where multiple devices are on a local bus.
//!
//! For more information see https://wiki.osdev.org/Pci

use crate::arch::port;

mod base_address;
mod device;
mod location;

pub use self::{base_address::*, device::*, location::Location};

/// Returns an iterator that lists all available PCI devices.
pub fn scan() -> impl Iterator<Item = Device> {
    Scanner {
        location: Some(Location::default()),
    }
}

struct Scanner {
    location: Option<Location>,
}

impl Iterator for Scanner {
    type Item = Device;

    fn next(&mut self) -> Option<Device> {
        let mut location = self.location.take()?;

        // loop until we find a location that doesn't have 0xFFFF as a vendor ID
        let (vendor_id, device_id) = loop {
            let (vendor_id, device_id) = read_location_u16(location);

            if vendor_id == 0xFFFF {
                location = location.next()?;
            } else {
                break (vendor_id, device_id);
            }
        };

        self.location = location.next();

        Device::read(vendor_id, device_id, location)
    }
}

pub(self) fn read_location_u32(location: Location) -> u32 {
    unsafe { port::write_u32(0xCF8, location.0 & 0b11111111_11111111_11111111_11111100) };
    let result = unsafe { port::read_u32(0xCFC) };

    result
}

pub(self) fn read_location_u16(location: Location) -> (u16, u16) {
    let result = read_location_u32(location);

    (result as u16, (result >> 16) as u16)
}

pub(self) fn read_location_u8(location: Location) -> (u8, u8, u8, u8) {
    let result = read_location_u32(location);

    (
        result as u8,
        (result >> 8) as u8,
        (result >> 16) as u8,
        (result >> 24) as u8,
    )
}
