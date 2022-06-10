#[repr(transparent)]
pub struct Volatile<T>(T);

impl<T: Copy> Volatile<T> {
    pub fn read(&self) -> T {
        unsafe { core::ptr::read_volatile(&self.0) }
    }
    pub fn write(&mut self, val: T) {
        unsafe { core::ptr::write_volatile(&mut self.0, val) }
    }
}

pub struct Peripherals {
    pub read: Volatile<Read>,
    _padding_1: [u32; 3],
    pub peek: Volatile<Peek>,
    pub sender: Volatile<Sender>,
    pub status: Volatile<Status>,
    pub config: Volatile<Config>,
    pub write: Volatile<Write>,
}

#[test]
fn test_offsets() {
    assert_eq!(memoffset::offset_of!(Peripherals, read), 0x00);
    assert_eq!(memoffset::offset_of!(Peripherals, peek), 0x10);
    assert_eq!(memoffset::offset_of!(Peripherals, sender), 0x14);
    assert_eq!(memoffset::offset_of!(Peripherals, status), 0x18);
    assert_eq!(memoffset::offset_of!(Peripherals, config), 0x1c);
    assert_eq!(memoffset::offset_of!(Peripherals, write), 0x20);
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Read(u32);

impl Read {
    // pub fn new(data: u32, channel: u8) -> Self {
    //     let mut result = Self(0);
    //     result.set_data(data);
    //     result.set_channel(channel);
    //     result
    // }
    #[cfg(test)]
    pub fn set_data(&mut self, data: u32) {
        self.0 = (self.0 & 0xf) | (data & 0xffff_fff0);
    }
    #[cfg(test)]
    pub fn set_channel(&mut self, channel: u8) {
        self.0 = (self.0 & 0xffff_fff0) | u32::from(channel & 0xf);
    }
    pub fn data(self) -> u32 {
        self.0 & 0xffff_fff0
    }
    pub fn channel(self) -> u8 {
        (self.0 & 0x0000_000f) as u8
    }
}

#[test]
fn test_read() {
    assert_eq!(core::mem::size_of::<Read>(), core::mem::size_of::<u32>());
    let mut read = Read(0);
    read.set_channel(0xf);
    read.set_data(0x1234_5670);

    assert_eq!(read.0, 0x1234_567F);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Peek(u32);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sender(u32);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Status(u32);

impl Status {
    pub fn full(self) -> bool {
        (self.0 & 0x8000_0000) > 0
    }
    pub fn empty(self) -> bool {
        (self.0 & 0x4000_0000) > 0
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Config(u32);

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Write(u32);

impl Write {
    pub fn new(data: u32, channel: u8) -> Self {
        let mut result = Self(0);
        result.set_data(data);
        result.set_channel(channel);
        result
    }
    pub fn set_data(&mut self, data: u32) {
        self.0 = (self.0 & 0xf) | (data & 0xffff_fff0);
    }
    pub fn set_channel(&mut self, channel: u8) {
        self.0 = (self.0 & 0xffff_fff0) | u32::from(channel & 0xf);
    }
    // pub fn data(&self) -> u32 {
    //     self.0 & 0xffff_fff0
    // }
    // pub fn channel(&self) -> u8 {
    //     (self.0 & 0x0000_000f) as u8
    // }
}

#[test]
fn test_write() {
    assert_eq!(core::mem::size_of::<Write>(), core::mem::size_of::<u32>());
    let mut write = Write(0);
    write.set_channel(0x0f);
    write.set_data(0x1234_5670);

    assert_eq!(write.0, 0x1234_567F);
}
