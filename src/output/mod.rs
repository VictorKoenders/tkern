pub use videocore_mailbox::Color;

use self::{framebuffer::FrameBufferOutput, qemu::QemuOutput};
use core::fmt::Write;
use utils::atomic_mutex;

pub mod framebuffer;
pub mod qemu;

pub fn print(cb: impl FnOnce(&mut Writer<'_>)) {
    let mut writer = Writer {
        fb: framebuffer::FRAMEBUFFER.lock(),
        qemu: &mut QemuOutput,
    };
    cb(&mut writer);
    if let Some(fb) = writer.fb.as_mut() {
        fb.reset_color();
    }
}

pub fn info(cb: impl FnOnce(&mut Writer<'_>)) {
    print(|w| {
        w.set_color(Color::INFO);
        let time = crate::time::Time::uptime();
        let _ = core::fmt::Write::write_fmt(
            w,
            format_args!(
                "[{:05}.{:03} INFO] ",
                time.as_secs(),
                time.as_millis() % 1000,
            ),
        );
        cb(w);
    });
}

#[allow(dead_code)]
pub fn warn(cb: impl FnOnce(&mut Writer<'_>)) {
    print(|w| {
        w.set_color(Color::WARN);
        let time = crate::time::Time::uptime();
        let _ = core::fmt::Write::write_fmt(
            w,
            format_args!(
                "[{:05}.{:03} WARN] ",
                time.as_secs(),
                time.as_millis() % 1000,
            ),
        );
        cb(w);
    });
}

pub struct Writer<'a> {
    fb: atomic_mutex::Guard<'a, Option<FrameBufferOutput>>,
    qemu: &'a mut QemuOutput,
}

impl<'a> Writer<'a> {
    pub fn set_color(&mut self, color: Color) {
        if let Some(fb) = self.fb.as_mut() {
            fb.set_color(color);
        }
    }
}

impl<'a> Write for Writer<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if let Some(fb) = self.fb.as_mut() {
            fb.write_str(s)?;
        }
        self.qemu.write_str(s)?;
        Ok(())
    }
}
