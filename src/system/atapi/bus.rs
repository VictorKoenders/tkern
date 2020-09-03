use crate::arch::port;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Bus {
    Primary,
    Secondary,
}

macro_rules! bus_fn {
    (
        $(#[$get_attr:meta])*
        $get:ident,
        $(#[$set_attr:meta])*
        $set:ident,
        $offset:expr
    ) => {
        $(#[$get_attr])*
        pub(super) fn $get(&self) -> u8 {
            unsafe { port::read_u8(self.address($offset)) }
        }
        $(#[$set_attr])*
        pub(super) fn $set(&self, val: u8) {
            unsafe { port::write_u8(self.address($offset), val) };
        }
    }
}

impl Bus {
    fn address(&self, offset: u16) -> u16 {
        match self {
            Bus::Primary => 0x1F0 + offset,
            Bus::Secondary => 0x170 + offset,
        }
    }

    bus_fn! { sector_count, set_sector_count, 2 }
    bus_fn! { lba_low, set_lba_low, 3 }
    bus_fn! { lba_mid, set_lba_mid, 4 }
    bus_fn! { lba_high, set_lba_high, 5 }
    bus_fn! { #[allow(dead_code)] drive, set_drive, 6 }

    pub(super) fn error(&self) -> Error {
        Error::from_bits_truncate(unsafe { port::read_u8(self.address(1)) })
    }

    pub(super) fn features(&self, features: Features) {
        unsafe { port::write_u8(self.address(1), features.bits()) }
    }

    pub(super) fn status(&self) -> Status {
        Status::from_bits_truncate(unsafe { port::read_u8(self.address(7)) })
    }

    pub(super) fn command(&self, command: Command) {
        unsafe { port::write_u8(self.address(7), command.0) }
    }

    pub(super) unsafe fn write_data(&self, data: &[u8]) {
        for data in data.chunks(2) {
            let u16_val = ((data[0] as u16) << 8) | (data[1] as u16);
            port::write_u16(self.address(0), u16_val);
        }
    }
}

bitflags::bitflags! {
    pub struct Error: u8 {
        const ADDRESS_MARK_NOT_FOUND = 0x01;
        const TRACK_ZERO_NOT_FOUND = 0x02;
        const ABORTED = 0x04;
        const MEDIA_CHANGE_REQUEST = 0x08;
        const ID_NOT_FOUND = 0x10;
        const MEDIA_CHANGED = 0x20;
        const UNCORRECTABLE_DATA_ERROR = 0x40;
        const BAD_BLOCK_DETECTED = 0x80;
    }
}

bitflags::bitflags! {
    pub struct Features: u8 {
        const NONE = 0x0;
    }
}

bitflags::bitflags! {
    pub struct Status: u8 {
        const ERROR = 0x01;
        const INDEX = 0x02;
        const CORRECTED = 0x04;
        const DATA_READY = 0x08; // DRQ
        const SERVICE_REQUEST = 0x10;
        const DRIVE_FAULT = 0x20;
        const READY = 0x40;
        const BUSY = 0x80;
    }
}

impl Status {
    pub fn as_err(self, bus: Bus) -> Result<Self, super::Error> {
        if self.contains(Status::ERROR) {
            let error = bus.error();
            Err(error.into())
        } else {
            Ok(self)
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Command(u8);

impl Command {
    pub const ATA_PACKET: Command = Command(0xA0);
    pub const IDENTIFY: Command = Command(0xEC);
}
