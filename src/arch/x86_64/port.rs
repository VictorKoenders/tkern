//! Generic port logic. Used for reading and writing data from ports on the CPU.

/// Reads a given value from a given port
///
/// # Safety
///
/// This function is unsafe because the I/O port could have side effects that violate memory
/// safety.
pub unsafe fn read_u8(port: u16) -> u8 {
    x86_64::instructions::port::PortRead::read_from_port(port)
}

/// Reads a given slice from a given port
///
/// # Safety
///
/// This function is unsafe because the I/O port could have side effects that violate memory
/// safety.
pub unsafe fn read_u8_slice(port: u16, buffer: &mut [u8]) {
    for b in buffer {
        *b = read_u8(port);
    }
}

/// Reads a given value from a given port
///
/// # Safety
///
/// This function is unsafe because the I/O port could have side effects that violate memory
/// safety.
pub unsafe fn read_u16(port: u16) -> u16 {
    x86_64::instructions::port::PortRead::read_from_port(port)
}

/// Reads a given slice from a given port
///
/// # Safety
///
/// This function is unsafe because the I/O port could have side effects that violate memory
/// safety.
pub unsafe fn read_u16_slice(port: u16, buffer: &mut [u16]) {
    for b in buffer {
        *b = read_u16(port);
    }
}

/// Reads a given value from a given port
///
/// # Safety
///
/// This function is unsafe because the I/O port could have side effects that violate memory
/// safety.
pub unsafe fn read_u32(port: u16) -> u32 {
    x86_64::instructions::port::PortRead::read_from_port(port)
}

/// Writes a given value to a given port
///
/// # Safety
///
/// This function is unsafe because the I/O port could have side effects that violate memory
/// safety.
pub unsafe fn write_u8(port: u16, val: u8) {
    x86_64::instructions::port::PortWrite::write_to_port(port, val)
}

/// Writes a given value to a given port
///
/// # Safety
///
/// This function is unsafe because the I/O port could have side effects that violate memory
/// safety.
pub unsafe fn write_u16(port: u16, val: u16) {
    x86_64::instructions::port::PortWrite::write_to_port(port, val)
}

/// Writes a given value to a given port
///
/// # Safety
///
/// This function is unsafe because the I/O port could have side effects that violate memory
/// safety.
pub unsafe fn write_u32(port: u16, val: u32) {
    x86_64::instructions::port::PortWrite::write_to_port(port, val)
}

/// Writes a given u16 slice to a given port. This is identical to:
///
/// ```rust
/// for b in slice {
///     write_u16(port, *b);
/// }
/// ```
///
/// # Safety
///
/// This function is unsafe because the I/O port could have side effects that violate memory
/// safety.
pub unsafe fn write_u16_slice(port: u16, slice: &[u16]) {
    for b in slice {
        write_u16(port, *b);
    }
}
