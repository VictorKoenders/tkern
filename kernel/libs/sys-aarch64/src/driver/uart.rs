use super::Uart;
use super::UART_ADDR;
use core::fmt;
use driver::uart;

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            uart::Write::byte(self, c);
        }
        Ok(())
    }
}

impl uart::Write for Uart {
    fn byte(&self, byte: u8) {
        unsafe { core::ptr::write_volatile(UART_ADDR, byte) }
    }
    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result {
        fmt::write(&mut Uart {}, args)
    }
    fn flush(&self) {}
}

impl uart::Read for Uart {
    fn char(&self) -> char {
        0 as char
    }

    fn clear(&self) {}
}