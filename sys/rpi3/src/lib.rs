#![cfg_attr(not(test), no_std)]
#![feature(strict_provenance)]
#![warn(unsafe_op_in_unsafe_fn, clippy::pedantic, rust_2018_idioms)]

#[macro_use]
mod macros;

mod pac;

pub struct MiniUart {}

fn delay(count: usize) {
    for _ in 0..count {
        cortex_a::asm::nop();
    }
}

impl MiniUart {
    pub fn init(aux: &mut pac::AUX::Peripheral, gpio: &mut pac::GPIO::Peripheral) -> Self {
        use pac::AUX::{BaudRate, UartDataSize};
        use pac::GPIO::FSEL;

        gpio.GPFSEL1
            .write(|w| w.fsel14().set(FSEL::Alt5).fsel15().set(FSEL::Alt5));

        gpio.GPPUD.clear();
        delay(150);
        gpio.GPPUDCLK0
            .write(|w| w.pudclk14().set().pudclk15().set());
        delay(150);
        gpio.GPPUDCLK0.clear();

        aux.ENB.write(|w| w.mini_uart_enable().set());
        aux.MU_CNTL_REG.clear();
        aux.MU_IER_REG.clear();
        aux.MU_LCR_REG
            .write(|w| w.data_size().set(UartDataSize::_8Bit));
        aux.MU_MCR_REG.clear();
        aux.MU_BAUD_REG
            .write(|w| w.baudrate().set(BaudRate::_115200));
        aux.MU_CNTL_REG
            .write(|w| w.transmitter_enable().set().receiver_enable().set());

        Self {}
    }
}
