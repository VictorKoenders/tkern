use core::fmt;

struct UartConsole;

impl fmt::Write for UartConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            unsafe { core::ptr::write_volatile(crate::bsp::UART_ADDR, c as u8) }
        }
        Ok(())
    }
}

pub fn get() -> impl super::Write {
    UartConsole
}
