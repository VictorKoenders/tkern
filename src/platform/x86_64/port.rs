use x86_64::instructions::port::{PortRead, PortWrite};

/// Write to a CPU port
///
/// # Safety
///
/// Anything can happen when you write a value to this port.
/// In the future we plan on making a safe layer around this, but for now the ports are exposed freely.
/// Use at own risk.
pub unsafe fn write_u8(port: u16, value: u8) {
    PortWrite::write_to_port(port, value);
}

/// Write to a CPU port
///
/// # Safety
///
/// Anything can happen when you write a value to this port.
/// In the future we plan on making a safe layer around this, but for now the ports are exposed freely.
/// Use at own risk.
pub unsafe fn write_u16(port: u16, value: u16) {
    PortWrite::write_to_port(port, value);
}

/// Write to a CPU port
///
/// # Safety
///
/// Anything can happen when you write a value to this port.
/// In the future we plan on making a safe layer around this, but for now the ports are exposed freely.
/// Use at own risk.
pub unsafe fn write_u32(port: u16, value: u32) {
    PortWrite::write_to_port(port, value);
}

/// Read from a CPU port
///
/// # Safety
///
/// Anything can happen when you read a value from this port.
/// In the future we plan on making a safe layer around this, but for now the ports are exposed freely.
/// Use at own risk.
pub unsafe fn read_u8(port: u16) -> u8 {
    PortRead::read_from_port(port)
}

/// Read from a CPU port
///
/// # Safety
///
/// Anything can happen when you read a value from this port.
/// In the future we plan on making a safe layer around this, but for now the ports are exposed freely.
/// Use at own risk.
pub unsafe fn read_u16(port: u16) -> u16 {
    PortRead::read_from_port(port)
}

/// Read from a CPU port
///
/// # Safety
///
/// Anything can happen when you read a value from this port.
/// In the future we plan on making a safe layer around this, but for now the ports are exposed freely.
/// Use at own risk.
pub unsafe fn read_u32(port: u16) -> u32 {
    PortRead::read_from_port(port)
}
