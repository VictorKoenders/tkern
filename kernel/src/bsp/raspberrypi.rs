use core::cell::Cell;

#[path = "raspberrypi/cpu.rs"]
pub mod cpu;

pub const UART_ADDR: *mut u8 = 0x3F20_1000 as *mut u8;

pub fn uart() -> &'static dyn crate::driver::uart::All {
    &UART
}

pub fn time() -> &'static dyn crate::driver::time::Time {
    if !TIME_MANAGER.is_inited() {
        TIME_MANAGER.init();
    }
    &TIME_MANAGER
}

/// Board identification.
pub fn board_name() -> &'static str {
    "Raspberry Pi 4"
}

static UART: Uart = Uart {};
struct Uart {}

mod uart_impl;

struct TimeManager {
    freq: Cell<u64>,
}

static TIME_MANAGER: TimeManager = TimeManager { freq: Cell::new(0) };
unsafe impl Sync for TimeManager {}

mod impl_time;
