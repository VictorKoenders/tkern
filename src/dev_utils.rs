//! Developer utility functions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
/// Exit the qemu virtual machine with this result
pub enum QemuExitCode {
    /// Ran succesfully
    Success = 0x10,
    /// Ran into a fatal error
    Failed = 0x11,
}

/// Exit the qemu virtual machine.
///
/// # Safety
///
/// Calling this in any different environment is considered UB
pub unsafe fn exit_qemu(code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0xf4);
    port.write(code as u32);
}
