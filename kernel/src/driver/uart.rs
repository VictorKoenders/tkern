use core::fmt;

pub trait Read {
    fn char(&self) -> char;
    fn clear(&self);
}

pub trait Write {
    fn char(&self, c: char) {
        let mut buff = [0u8; 4];
        let str = c.encode_utf8(&mut buff);
        for byte in str.bytes() {
            self.byte(byte);
        }
    }
    fn byte(&self, b: u8);
    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
    fn flush(&self);
}

pub trait All: Read + Write {}

impl<T> All for T where T: Read + Write {}
