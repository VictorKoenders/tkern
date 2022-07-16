#![allow(
    clippy::upper_case_acronyms,
    dead_code,
    non_snake_case,
    non_camel_case_types
)]

use core::ptr::NonNull;

pub mod AUX {
    //! There are two Auxiliary registers which control all three devices. One is the interrupt status
    //! register, the second is the Auxiliary enable register. The Auxiliary IRQ status register can
    //! help to hierarchically determine the source of an interrupt.

    /// The AUXIRQ register is used to check any pending interrupts which may be asserted by the three Auxiliary sub blocks.
    #[derive(sys_derive::Peripheral)]
    #[peripheral(address = 0x21_5000)]
    pub struct IRQ {
        #[field(bit = 2, readable)]
        /// If set the SPI 2 module has an interrupt pending.
        pub SPI_2_IRQ: bool,
        #[field(bit = 1, readable)]
        /// If set the SPI 1 module has an interrupt pending.
        pub SPI_1_IRQ: bool,
        #[field(bit = 0, readable)]
        /// If set the mini UART has an interrupt pending.
        pub MINI_UART_IRQ: bool,
    }

    /// The AUXENB register is used to enable the three modules; UART, SPI1, SPI2.
    #[derive(sys_derive::Peripheral)]
    #[peripheral(address = 0x21_5004)]
    pub struct ENB {
        /// If set the SPI 2 module is enabled.  If clear the SPI 2 module is disabled. That also disables any SPI 2 module register access
        #[field(bit = 2, readable, writable)]
        pub SPI2_enable: bool,
        /// If set the SPI 1 module is enabled.  If clear the SPI 1 module is disabled. That also disables any SPI 1 module register access
        #[field(bit = 1, readable, writable)]
        pub SPI1_enable: bool,
        /// If set the mini UART is enabled. The UART will immediately start receiving data, especially if the UART1_RX line is low.
        /// If clear the mini UART is disabled. That also disables any mini UART register acces
        #[field(bit = 0, readable, writable)]
        pub Mini_UART_enable: bool,
    }

    /// The `AUX_MU_IO_REG` register is primary used to write data to and read data from the
    /// UART FIFOs.
    /// If the DLAB bit in the line control register is set this register gives access to the LS 8 bits
    /// of the baud rate. (Note: there is easier access to the baud rate register)
    #[derive(sys_derive::Peripheral)]
    #[peripheral(address = 0x21_5040)]
    pub struct MU_IO_REG {
        /// Access to the LS 8 bits of the 16-bit baudrate register.
        /// (Only If bit 7 of the line control register (DLAB bit) is set)
        #[field(bits = 7:0, readable, writable)]
        pub LS_8_bit_baudrate: u8,
        /// Data written is put in the transmit FIFO (Provided it is not full)
        // (Only If bit 7 of the line control register (DLAB bit) is clear)
        #[field(bits = 7:0, writable)]
        pub transmit_data_write: u8,
        /// Data read is taken from the receive FIFO (Provided it is not empty)
        /// (Only If bit 7 of the line control register (DLAB bit) is clear)
        #[field(bits = 7:0, readable)]
        pub receive_data_read: u8,
    }

    /// The `AUX_MU_IER_REG` register is primary used to enable interrupts
    /// If the DLAB bit in the line control register is set this register gives access to the MS 8 bits
    /// of the baud rate. (Note: there is easier access to the baud rate register)
    #[derive(sys_derive::Peripheral)]
    #[peripheral(address = 0x21_5044)]
    pub struct MU_IER_REG {
        /// Access to the MS 8 bits of the 16-bit baudrate register.
        /// (Only If bit 7 of the line control register (DLAB bit) is set)
        #[field(bits = 7:0, readable, writable)]
        pub MS_8_bits_baudrate: u8,

        /// Must be set to 0b11 to receive interrupts
        #[field(bits = 3:2, writable)]
        pub enable_interrupts: u8,

        /// If this bit is set the interrupt line is asserted whenever the transmit FIFO is empty.
        /// If this bit is clear no transmit interrupts are generated.
        #[field(bit = 1, readable, writable)]
        pub Enable_transmit_interrupts: bool,

        /// If this bit is set the interrupt line is asserted whenever the receive FIFO holds at least 1 byte
        /// If this bit is clear no receive interrupts are generated.
        #[field(bit = 1, readable, writable)]
        pub Enable_receive_interrupts: bool
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
    ///
    /// [`R`]: #associatedtype.R
    unsafe fn read(base_address: NonNull<()>) -> Self::R;

    /// Write the value [`W`] to the register.
    ///
    /// # Safety
    ///
    /// The base address must be a valid address. For RPI2 this will be `0x2000_0000`, for RPI3 and newer this will be `0x3F00_000`
    ///
    /// [`W`]: #associatedtype.W
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
