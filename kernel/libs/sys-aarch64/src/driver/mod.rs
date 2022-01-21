use core::cell::Cell;

mod time;
mod uart;

pub static UART: Uart = Uart {};
pub struct Uart {}

pub struct TimeManager {
    freq: Cell<u64>,
}

pub static TIME_MANAGER: TimeManager = TimeManager { freq: Cell::new(0) };
unsafe impl Sync for TimeManager {}

const UART_ADDR: *mut u8 = 0x3F20_1000 as *mut u8;
