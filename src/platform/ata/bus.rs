use crate::platform::port;
use bitflags::bitflags;
use core::ops::Range;

#[repr(u16)]
enum Write {
    Data = 0,
    Features = 1,
    SectorCount = 2,
    LbaLow = 3,
    LbaMid = 4,
    LbaHigh = 5,
    DriveSelect = 6,
    Command = 7,
}

impl Write {
    fn write(self, bus: &Bus, value: u8) {
        unsafe {
            port::write_u8(bus.io_base() + (self as u16), value);
        }
    }
}

#[repr(u16)]
enum Read {
    Data = 0,
    Error = 1,
    SectorCount = 2,
    LbaLow = 3,
    LbaMid = 4,
    LbaHigh = 5,
    DriveSelect = 6,
    Status = 7,
}

impl Read {
    fn read(self, bus: &Bus) -> u8 {
        unsafe { port::read_u8(bus.io_base() + (self as u16)) }
    }
}

#[repr(u8)]
pub enum Command {
    Identify = 0xEC,
}

// TODO: If the next two buses exist, they are normally controlled by IO ports 0x1E8 through 0x1EF, and 0x168 through 0x16F, respectively.
// The associated Device Control Registers/Alternate Status ports are IO ports 0x3E6, and 0x366.

pub struct Bus {
    io_range: Range<u16>,
    status: u16,
    irq: u8,
}

impl Bus {
    pub const PRIMARY: &'static Bus = &Bus {
        io_range: 0x1F0..0x1F7,
        status: 0x3F6,
        irq: 14,
    };
    pub const SECONDARY: &'static Bus = &Bus {
        io_range: 0x170..0x177,
        status: 0x376,
        irq: 15,
    };

    fn io_base(&self) -> u16 {
        self.io_range.start
    }

    pub fn set_drive(&self, drive: u8) {
        Write::DriveSelect.write(self, drive);
    }
    pub fn set_sector_count(&self, sector_count: u8) {
        Write::SectorCount.write(self, sector_count);
    }
    pub fn get_sector_count(&self) -> u8 {
        Read::SectorCount.read(self)
    }
    pub fn set_lba_low(&self, lba: u8) {
        Write::LbaLow.write(self, lba);
    }
    pub fn get_lba_low(&self) -> u8 {
        Read::LbaLow.read(self)
    }
    pub fn set_lba_mid(&self, lba: u8) {
        Write::LbaMid.write(self, lba);
    }
    pub fn get_lba_mid(&self) -> u8 {
        Read::LbaMid.read(self)
    }
    pub fn set_lba_high(&self, lba: u8) {
        Write::LbaHigh.write(self, lba);
    }
    pub fn get_lba_high(&self) -> u8 {
        Read::LbaHigh.read(self)
    }
    pub fn send_command(&self, command: Command) {
        Write::Command.write(self, command as u8);
    }
    pub fn get_raw_status(&self) -> u8 {
        Read::Status.read(self)
    }
    pub fn get_status(&self) -> Status {
        Status::from_bits_truncate(self.get_raw_status())
    }
}

bitflags! {
    pub struct Status: u8 {
        const ERROR = 0x01;
        const INDEX = 0x02;
        const CORRECTED = 0x04;
        const DRQ = 0x08;
        const SRV = 0x10;
        const DF = 0x20;
        const RDY = 0x40;
        const BUSY = 0x80;
    }
}
