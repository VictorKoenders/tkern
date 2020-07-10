use x86_64::instructions::port::{PortRead, PortWrite};

pub unsafe fn write_u8(port: u16, value: u8) {
    PortWrite::write_to_port(port, value);
}

pub unsafe fn write_u16(port: u16, value: u16) {
    PortWrite::write_to_port(port, value);
}

pub unsafe fn write_u32(port: u16, value: u32) {
    PortWrite::write_to_port(port, value);
}

pub unsafe fn read_u8(port: u16) -> u8 {
    PortRead::read_from_port(port)
}

pub unsafe fn read_u16(port: u16) -> u16 {
    PortRead::read_from_port(port)
}

pub unsafe fn read_u32(port: u16) -> u32 {
    PortRead::read_from_port(port)
}
