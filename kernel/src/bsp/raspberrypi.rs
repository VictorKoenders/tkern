#[path = "raspberrypi/cpu.rs"]
pub mod cpu;

pub const UART_ADDR: *mut u8 = 0x3F20_1000 as *mut u8;

pub fn uart() -> &'static dyn crate::driver::uart::All {
    &UART
}

static UART: Uart = Uart {};

struct Uart {}
mod uart_impl {
    use super::Uart;
    use crate::driver::uart;
    use core::fmt;

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
            unsafe { core::ptr::write_volatile(super::UART_ADDR, byte) }
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
}
