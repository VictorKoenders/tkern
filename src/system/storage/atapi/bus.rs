use crate::arch::port;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Bus(u16);

impl core::fmt::Debug for Bus {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "Bus(0x{:04X})", self.0)
    }
}
impl Bus {
    /// Create a new bus at the given address
    ///
    /// # Safety
    ///
    /// The caller must ensure the address is correct.
    /// Writing data to wrong addresses can cause extreme undefined behavior.
    pub unsafe fn new(address: u16) -> Bus {
        Bus(address)
    }
    pub fn ata_primary() -> Bus {
        Bus(0x1F0)
    }

    pub fn ata_secondary() -> Bus {
        Bus(0x170)
    }
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
    pub fn address(&self, offset: u16) -> u16 {
        self.0 + offset
    }

    bus_fn! {
        #[allow(dead_code)] sector_count,
        #[allow(dead_code)] set_sector_count,
        2
    }
    bus_fn! {
        #[allow(dead_code)] lba_low,
        #[allow(dead_code)] set_lba_low,
        3
    }
    bus_fn! {
        #[allow(dead_code)] lba_mid,
        #[allow(dead_code)] set_lba_mid,
        4
    }
    bus_fn! {
        #[allow(dead_code)] lba_high,
        #[allow(dead_code)] set_lba_high,
        5
    }
    bus_fn! {
        #[allow(dead_code)] drive,
        #[allow(dead_code)] set_drive,
        6
    }

    bus_fn! {
        #[allow(dead_code)] address_1,
        #[allow(dead_code)] set_address_1,
        3
    }
    bus_fn! {
        #[allow(dead_code)] address_2,
        #[allow(dead_code)] set_address_2,
        4
    }
    bus_fn! {
        #[allow(dead_code)] address_3,
        #[allow(dead_code)] set_address_3,
        5
    }

    pub(super) fn error(&self) -> Error {
        Error::from_bits_truncate(unsafe { port::read_u8(self.address(1)) })
    }

    pub(super) fn features(&self, features: Features) {
        unsafe { port::write_u8(self.address(1), features.bits()) }
    }

    pub(super) fn status(&self) -> Status {
        Status::from_bits_truncate(unsafe { port::read_u8(self.address(7)) })
    }

    pub(super) fn command(&self, command: u8) {
        // command: Command) {
        unsafe { port::write_u8(self.address(7), command) } // command.0) }
    }

    pub(super) fn device_control_register(&self) -> u8 {
        unsafe { port::read_u8(self.address(0x206)) }
    }

    /// Writes the given buffer to the data bus
    ///
    /// # Safety
    ///
    /// The given data slice MUST be on a u16 aligned boundary, as it is send as an u16 slice.
    /// Also all ATA related semantics must be kept in mind when calling this function.
    pub(super) unsafe fn write_data(&self, data: &[u8]) {
        let data = core::slice::from_raw_parts(data.as_ptr() as *const u16, data.len() / 2);
        port::write_u16_slice(self.address(0), data);
    }

    /// Reads data from the data bus to fill the given buffer
    ///
    /// # Safety
    ///
    /// The given data slice MUST be on a u16 aligned boundary, as it is read as an u16 slice.
    /// Also all ATA related semantics must be kept in mind when calling this function.
    pub(super) unsafe fn read_data(&self, data: &mut [u8]) {
        let data = core::slice::from_raw_parts_mut(data.as_ptr() as *mut u16, data.len() / 2);
        port::read_u16_slice(self.address(0), data);
    }

    pub fn wait_ready(&self) {
        loop {
            let status = self.status();
            if status.contains(Status::DEVICE_READY) {
                return;
            }
            core::hint::spin_loop();
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
        const DATA_CORRECTED = 0x04;
        const READ_AVAILABLE = 0x08;
        const SEEK_COMPLETE = 0x10;
        const DEVICE_FAULT = 0x20;
        const DEVICE_READY = 0x40;
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
    #[allow(dead_code)]
    pub const ATA_PACKET: Command = Command(0xA0);
    #[allow(dead_code)]
    pub const IDENTIFY: Command = Command(0xEC);
}
