#[path = "raspberrypi/cpu.rs"]
pub mod cpu;

pub const UART_ADDR: *mut u8 = 0x3F20_1000 as *mut u8;
