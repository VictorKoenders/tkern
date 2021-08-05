mod bus;
pub mod identify;
mod inner;

use crate::interrupts::StackFrame;
use core::num::NonZeroU16;

#[allow(dead_code)]
pub const SECTOR_SIZE: usize = 2048;
pub use self::bus::Bus;

use self::{identify::Identify, inner::INNER};

pub fn test(address: Option<u16>) {
    crate::interrupts::register_custom_interrupt(NonZeroU16::new(92).unwrap(), atapi_interrupt);
    let bus = if let Some(a) = address {
        unsafe { Bus::new(a) }
    } else {
        Bus::ata_primary()
    };
    match identify(bus) {
        Ok(identify) => {
            vga_println!("ATAPI Identify loaded:");
            vga_println!("  {:?}", &{ identify.config });
            vga_println!("  {:?}", &{ identify.capabilities });
            vga_println!("  {:?}", &{ identify.command_set_supported });
        }
        Err(Error::BusSelect) => {
            vga_println!("Normal ATAPI did not work, trying SATA");
            // Read signature bytes
            let signature_byte_1 = bus.lba_mid();
            let signature_byte_2 = bus.lba_high();

            vga_println!(
                "Signature bytes: 0x{:02X} and 0x{:02X}",
                signature_byte_1,
                signature_byte_2
            );
        }
        Err(e) => {
            vga_println!("Could not identify ATAPI: {:?}", e);
        }
    }
}

pub fn identify(bus: Bus) -> Result<Identify, Error> {
    bus.set_drive(0xA0);
    bus.set_sector_count(0x0);
    bus.set_lba_low(0x0);
    bus.set_lba_mid(0x0);
    bus.set_lba_high(0x0);
    bus.command(0xEC);

    loop {
        let status = bus.status().as_err(bus)?;
        if !status.contains(bus::Status::BUSY) || status.contains(bus::Status::READ_AVAILABLE) {
            break;
        }
        crate::arch::halt();
    }
    let mut buffer = [0u8; 512];
    unsafe {
        bus.read_data(&mut buffer);
    }

    let interrupt_id = crate::interrupts::consume_last_unknown_interrupt_id();
    vga_println!("ATAPI interrupt is at ID {:?}", interrupt_id);
    if let Some(interrupt_id) = interrupt_id {
        crate::interrupts::register_custom_interrupt(interrupt_id, atapi_interrupt);
    }
    let interrupt_id = crate::interrupts::consume_last_unknown_interrupt_id();
    vga_println!("ATAPI interrupt now is at ID {:?}", interrupt_id);

    unsafe { Ok(core::mem::transmute(buffer)) }
}

custom_interrupt! {
    fn atapi_interrupt(_stack_frame: StackFrame) {
        vga_println!("ATAPI interrupt triggered");
    }
}

#[allow(dead_code)]
pub fn read_sector(
    bus: Bus,
    drive: u8,
    lba: u32,
    _buffer: &mut [u8; SECTOR_SIZE],
) -> Result<usize, Error> {
    let mut inner = INNER.lock();
    inner.drive_select(bus, drive)?;

    let command: [u8; 12] = [
        0xA8, // READ SECTORS
        0,
        ((lba >> 0x18) & 0xFF) as u8, // Most significant byte of LBA
        ((lba >> 0x10) & 0xFF) as u8,
        ((lba >> 0x08) & 0xFF) as u8,
        (lba & 0xFF) as u8, // Least significant byte of LBA
        0,
        0,
        0,
        1, // 1 sector
        0,
        0,
    ];

    unsafe { bus.write_data(&command) };

    // TODO: Wait for IRQ

    Ok(0)
}

#[derive(Debug)]
#[allow(missing_docs)]
pub enum Error {
    BusSelect,
    AddressMarkNotFound,
    TrackZeroNotFound,
    Aborted,
    MediaChangeRequest,
    IdNotFound,
    MediaChanged,
    UncorrectableData,
    BadBlock,
}

impl From<bus::Error> for Error {
    fn from(e: bus::Error) -> Self {
        if e.contains(bus::Error::ADDRESS_MARK_NOT_FOUND) {
            Error::AddressMarkNotFound
        } else if e.contains(bus::Error::TRACK_ZERO_NOT_FOUND) {
            Error::TrackZeroNotFound
        } else if e.contains(bus::Error::ABORTED) {
            Error::Aborted
        } else if e.contains(bus::Error::MEDIA_CHANGE_REQUEST) {
            Error::MediaChangeRequest
        } else if e.contains(bus::Error::ID_NOT_FOUND) {
            Error::IdNotFound
        } else if e.contains(bus::Error::MEDIA_CHANGED) {
            Error::MediaChanged
        } else if e.contains(bus::Error::UNCORRECTABLE_DATA_ERROR) {
            Error::UncorrectableData
        } else if e.contains(bus::Error::BAD_BLOCK_DETECTED) {
            Error::BadBlock
        } else {
            unreachable!("atapi::bus::Error is invalid: {:?}", e);
        }
    }
}
