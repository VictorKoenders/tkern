use core::fmt::{Arguments, Write};

pub mod framebuffer;
pub mod qemu;

pub fn print(args: Arguments<'_>) {
    let mut lock = framebuffer::FRAMEBUFFER.lock();
    if let Some(output) = lock.as_mut() {
        let _ = output.write_fmt(args);
    } else {
        // optimize for the `Some` path to succeed
        #[cold]
        fn unlikely() {}
        unlikely();
    }
    let _ = qemu::QemuOutput.write_fmt(args);
}
