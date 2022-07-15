#![allow(clippy::upper_case_acronyms, dead_code, non_snake_case)]

use core::ptr::NonNull;

pub mod AUX {
    //! There are two Auxiliary registers which control all three devices. One is the interrupt status
    //! register, the second is the Auxiliary enable register. The Auxiliary IRQ status register can
    //! help to hierarchically determine the source of an interrupt.

    /// The AUXIRQ register is used to check any pending interrupts which may be asserted by the three Auxiliary sub blocks.
    #[derive(sys_derive::Peripheral)]
    #[peripheral(address = 0x21_5000)]
    pub struct IRQ {
        #[field(bit = 2, readable, reset = false)]
        /// If set the SPI 2 module has an interrupt pending.
        pub SPI_2_IRQ: bool,
        #[field(bit = 1, readable, reset = false)]
        /// If set the SPI 1 module has an interrupt pending.
        pub SPI_1_IRQ: bool,
        #[field(bit = 0, readable, reset = false)]
        /// If set the mini UART has an interrupt pending.
        pub MINI_UART_IRQ: bool,
    }

    /// The AUXENB register is used to enable the three modules; UART, SPI1, SPI2.
    #[derive(sys_derive::Peripheral)]
    #[peripheral(address = 0x21_5004)]
    pub struct ENB {
        /// If set the SPI 2 module is enabled.  If clear the SPI 2 module is disabled. That also disables any SPI 2 module register access
        #[field(bit = 2, readable, writable, reset = 0)]
        pub SPI2_enable: bool,
        /// If set the SPI 1 module is enabled.  If clear the SPI 1 module is disabled. That also disables any SPI 1 module register access
        #[field(bit = 1, readable, writable, reset = 0)]
        pub SPI1_enable: bool,
        /// If set the mini UART is enabled. The UART will immediately start receiving data, especially if the UART1_RX line is low.  If clear the mini UART is disabled. That also disables any mini UART register acces
        #[field(bit = 0, readable, writable, reset = 0)]
        pub Mini_UART_enable: bool,
    }
}

pub trait Peripheral {
    type R;
    type W: Default;

    /// Read the value [`R`] from the register.
    ///
    /// # Safety
    ///
    /// The base address must be a valid address. For RPI2 this will be `0x2000_0000`, for RPI3 and newer this will be `0x3F00_000`
    unsafe fn read(base_address: NonNull<()>) -> Self::R;

    /// Write the value [`W`] to the register.
    ///
    /// # Safety
    ///
    /// The base address must be a valid address. For RPI2 this will be `0x2000_0000`, for RPI3 and newer this will be `0x3F00_000`
    unsafe fn write(base_address: NonNull<()>, write: Self::W);

    /// Modify the value of the register
    ///
    /// # Safety
    ///
    /// The base address must be a valid address. For RPI2 this will be `0x2000_0000`, for RPI3 and newer this will be `0x3F00_000`
    unsafe fn modify<F>(base_address: NonNull<()>, f: F)
    where
        F: FnOnce(Self::R, Self::W) -> Self::W,
    {
        unsafe {
            let read = Self::read(base_address);
            let write = Self::W::default();
            let write = f(read, write);
            Self::write(base_address, write);
        }
    }
}
