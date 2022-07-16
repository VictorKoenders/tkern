#![feature(prelude_import)]
#![no_std]
#![feature(strict_provenance)]
#![warn(unsafe_op_in_unsafe_fn, clippy::pedantic, rust_2018_idioms)]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
#[macro_use]
extern crate compiler_builtins;
#[macro_use]
mod macros {}
mod pac {
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
        pub mod irq {
            pub struct R(pub(super) [u32; 1]);
            impl R {
                /// If set the SPI 2 module has an interrupt pending.
                pub const fn spi_2_irq(&self) -> SPI_2_IRQ_R<'_> {
                    SPI_2_IRQ_R(self)
                }
                /// If set the SPI 1 module has an interrupt pending.
                pub const fn spi_1_irq(&self) -> SPI_1_IRQ_R<'_> {
                    SPI_1_IRQ_R(self)
                }
                /// If set the mini UART has an interrupt pending.
                pub const fn mini_uart_irq(&self) -> MINI_UART_IRQ_R<'_> {
                    MINI_UART_IRQ_R(self)
                }
            }
            pub struct SPI_2_IRQ_R<'a>(&'a R);
            impl SPI_2_IRQ_R<'_> {
                pub const fn is_set(&self) -> bool {
                    let mut bytes = (self.0).0;
                    let idx: usize = (bytes.len() - 1 - 0usize);
                    let offset: usize = 2usize;
                    (bytes[idx] & (1 << offset)) > 0
                }
                pub const fn is_clear(&self) -> bool {
                    !self.is_set()
                }
            }
            pub struct SPI_1_IRQ_R<'a>(&'a R);
            impl SPI_1_IRQ_R<'_> {
                pub const fn is_set(&self) -> bool {
                    let mut bytes = (self.0).0;
                    let idx: usize = (bytes.len() - 1 - 0usize);
                    let offset: usize = 1usize;
                    (bytes[idx] & (1 << offset)) > 0
                }
                pub const fn is_clear(&self) -> bool {
                    !self.is_set()
                }
            }
            pub struct MINI_UART_IRQ_R<'a>(&'a R);
            impl MINI_UART_IRQ_R<'_> {
                pub const fn is_set(&self) -> bool {
                    let mut bytes = (self.0).0;
                    let idx: usize = (bytes.len() - 1 - 0usize);
                    let offset: usize = 0usize;
                    (bytes[idx] & (1 << offset)) > 0
                }
                pub const fn is_clear(&self) -> bool {
                    !self.is_set()
                }
            }
            pub struct W(pub(super) [u32; 1]);
            impl Default for W {
                fn default() -> Self {
                    Self(Default::default())
                }
            }
            impl W {}
        }
        impl super::Peripheral for IRQ {
            type R = irq::R;
            type W = irq::W;
            unsafe fn read(base_address: core::ptr::NonNull<()>) -> Self::R {
                use core::num::NonZeroUsize;
                let bytes = unsafe {
                    core::ptr::read_volatile(
                        base_address
                            .map_addr(|addr| NonZeroUsize::new(addr.get() + 0x21_5000).unwrap())
                            .cast()
                            .as_ptr(),
                    )
                };
                irq::R(bytes)
            }
            unsafe fn write(base_address: core::ptr::NonNull<()>, value: Self::W) {
                use core::num::NonZeroUsize;
                let bytes = value.0;
                unsafe {
                    core::ptr::write_volatile(
                        base_address
                            .map_addr(|addr| NonZeroUsize::new(addr.get() + 0x21_5000).unwrap())
                            .cast()
                            .as_ptr(),
                        bytes,
                    );
                }
            }
        }
        /// The AUXENB register is used to enable the three modules; UART, SPI1, SPI2.
        #[peripheral(address = 0x21_5004)]
        pub struct ENB {
            /// If set the SPI 2 module is enabled.  If clear the SPI 2 module is disabled. That also disables any SPI 2 module register access
            #[field(bit = 2, readable, writable, reset = 0)]
            pub SPI2_enable: bool,
            /// If set the SPI 1 module is enabled.  If clear the SPI 1 module is disabled. That also disables any SPI 1 module register access
            #[field(bit = 1, readable, writable, reset = 0)]
            pub SPI1_enable: bool,
            /// If set the mini UART is enabled. The UART will immediately start receiving data, especially if the UART1_RX line is low.
            /// If clear the mini UART is disabled. That also disables any mini UART register acces
            #[field(bit = 0, readable, writable, reset = 0)]
            pub Mini_UART_enable: bool,
        }
        pub mod enb {
            pub struct R(pub(super) [u32; 1]);
            impl R {
                /// If set the SPI 2 module is enabled.  If clear the SPI 2 module is disabled. That also disables any SPI 2 module register access
                pub const fn spi2_enable(&self) -> SPI2_enable_R<'_> {
                    SPI2_enable_R(self)
                }
                /// If set the SPI 1 module is enabled.  If clear the SPI 1 module is disabled. That also disables any SPI 1 module register access
                pub const fn spi1_enable(&self) -> SPI1_enable_R<'_> {
                    SPI1_enable_R(self)
                }
                /// If set the mini UART is enabled. The UART will immediately start receiving data, especially if the UART1_RX line is low.
                /// If clear the mini UART is disabled. That also disables any mini UART register acces
                pub const fn mini_uart_enable(&self) -> Mini_UART_enable_R<'_> {
                    Mini_UART_enable_R(self)
                }
            }
            pub struct SPI2_enable_R<'a>(&'a R);
            impl SPI2_enable_R<'_> {
                pub const fn is_set(&self) -> bool {
                    let mut bytes = (self.0).0;
                    let idx: usize = (bytes.len() - 1 - 0usize);
                    let offset: usize = 2usize;
                    (bytes[idx] & (1 << offset)) > 0
                }
                pub const fn is_clear(&self) -> bool {
                    !self.is_set()
                }
            }
            pub struct SPI1_enable_R<'a>(&'a R);
            impl SPI1_enable_R<'_> {
                pub const fn is_set(&self) -> bool {
                    let mut bytes = (self.0).0;
                    let idx: usize = (bytes.len() - 1 - 0usize);
                    let offset: usize = 1usize;
                    (bytes[idx] & (1 << offset)) > 0
                }
                pub const fn is_clear(&self) -> bool {
                    !self.is_set()
                }
            }
            pub struct Mini_UART_enable_R<'a>(&'a R);
            impl Mini_UART_enable_R<'_> {
                pub const fn is_set(&self) -> bool {
                    let mut bytes = (self.0).0;
                    let idx: usize = (bytes.len() - 1 - 0usize);
                    let offset: usize = 0usize;
                    (bytes[idx] & (1 << offset)) > 0
                }
                pub const fn is_clear(&self) -> bool {
                    !self.is_set()
                }
            }
            pub struct W(pub(super) [u32; 1]);
            impl Default for W {
                fn default() -> Self {
                    Self(Default::default())
                        .spi2_enable()
                        .clear()
                        .spi1_enable()
                        .clear()
                        .mini_uart_enable()
                        .clear()
                }
            }
            impl W {
                /// If set the SPI 2 module is enabled.  If clear the SPI 2 module is disabled. That also disables any SPI 2 module register access
                pub const fn spi2_enable(self) -> SPI2_enable_W {
                    SPI2_enable_W(self)
                }
                /// If set the SPI 1 module is enabled.  If clear the SPI 1 module is disabled. That also disables any SPI 1 module register access
                pub const fn spi1_enable(self) -> SPI1_enable_W {
                    SPI1_enable_W(self)
                }
                /// If set the mini UART is enabled. The UART will immediately start receiving data, especially if the UART1_RX line is low.
                /// If clear the mini UART is disabled. That also disables any mini UART register acces
                pub const fn mini_uart_enable(self) -> Mini_UART_enable_W {
                    Mini_UART_enable_W(self)
                }
            }
            pub struct SPI2_enable_W(W);
            impl SPI2_enable_W {
                pub const fn set(self) -> W {
                    let mut bytes = (self.0).0;
                    let idx: usize = (bytes.len() - 1 - 0usize);
                    let offset: usize = 2usize;
                    bytes[idx] |= 1 << offset;
                    W(bytes)
                }
                pub const fn clear(self) -> W {
                    let mut bytes = (self.0).0;
                    let idx: usize = (bytes.len() - 1 - 0usize);
                    let offset: usize = 2usize;
                    bytes[idx] &= !(1 << offset);
                    W(bytes)
                }
            }
            pub struct SPI1_enable_W(W);
            impl SPI1_enable_W {
                pub const fn set(self) -> W {
                    let mut bytes = (self.0).0;
                    let idx: usize = (bytes.len() - 1 - 0usize);
                    let offset: usize = 1usize;
                    bytes[idx] |= 1 << offset;
                    W(bytes)
                }
                pub const fn clear(self) -> W {
                    let mut bytes = (self.0).0;
                    let idx: usize = (bytes.len() - 1 - 0usize);
                    let offset: usize = 1usize;
                    bytes[idx] &= !(1 << offset);
                    W(bytes)
                }
            }
            pub struct Mini_UART_enable_W(W);
            impl Mini_UART_enable_W {
                pub const fn set(self) -> W {
                    let mut bytes = (self.0).0;
                    let idx: usize = (bytes.len() - 1 - 0usize);
                    let offset: usize = 0usize;
                    bytes[idx] |= 1 << offset;
                    W(bytes)
                }
                pub const fn clear(self) -> W {
                    let mut bytes = (self.0).0;
                    let idx: usize = (bytes.len() - 1 - 0usize);
                    let offset: usize = 0usize;
                    bytes[idx] &= !(1 << offset);
                    W(bytes)
                }
            }
        }
        impl super::Peripheral for ENB {
            type R = enb::R;
            type W = enb::W;
            unsafe fn read(base_address: core::ptr::NonNull<()>) -> Self::R {
                use core::num::NonZeroUsize;
                let bytes = unsafe {
                    core::ptr::read_volatile(
                        base_address
                            .map_addr(|addr| NonZeroUsize::new(addr.get() + 0x21_5004).unwrap())
                            .cast()
                            .as_ptr(),
                    )
                };
                enb::R(bytes)
            }
            unsafe fn write(base_address: core::ptr::NonNull<()>, value: Self::W) {
                use core::num::NonZeroUsize;
                let bytes = value.0;
                unsafe {
                    core::ptr::write_volatile(
                        base_address
                            .map_addr(|addr| NonZeroUsize::new(addr.get() + 0x21_5004).unwrap())
                            .cast()
                            .as_ptr(),
                        bytes,
                    );
                }
            }
        }
        /// The AUX_MU_IO_REG register is primary used to write data to and read data from the
        /// UART FIFOs.
        /// If the DLAB bit in the line control register is set this register gives access to the LS 8 bits
        /// of the baud rate. (Note: there is easier access to the baud rate register)
        #[peripheral(address = 0x21_5040)]
        pub struct MU_IO_REG {
            /// Access to the LS 8 bits of the 16-bit baudrate register.
            /// (Only If bit 7 of the line control register (DLAB bit) is set)
            # [field (bits = 7 : 0 , readable , writable , reset = 0)]
            pub LS_8_bit_baudrate: u8,
            /// Data written is put in the transmit FIFO (Provided it is not full)
            # [field (bits = 7 : 0 , writable , reset = 0)]
            pub transmit_data_write: u8,
            /// Data read is taken from the receive FIFO (Provided it is not empty)
            /// (Only If bit 7 of the line control register (DLAB bit) is clear)
            # [field (bits = 7 : 0 , readable , reset = 0)]
            pub receive_data_read: u8,
        }
        pub mod mu_io_reg {
            pub struct R(pub(super) [u32; 1]);
            impl R {
                /// Access to the LS 8 bits of the 16-bit baudrate register.
                /// (Only If bit 7 of the line control register (DLAB bit) is set)
                pub const fn ls_8_bit_baudrate(&self) -> LS_8_bit_baudrate_R<'_> {
                    LS_8_bit_baudrate_R(self)
                }
                /// Data read is taken from the receive FIFO (Provided it is not empty)
                /// (Only If bit 7 of the line control register (DLAB bit) is clear)
                pub const fn receive_data_read(&self) -> receive_data_read_R<'_> {
                    receive_data_read_R(self)
                }
            }
            pub struct LS_8_bit_baudrate_R<'a>(&'a R);
            impl LS_8_bit_baudrate_R<'_> {
                pub const fn get_value(self) -> u8 {
                    let mut bytes = (self.0).0;
                    let idx: usize = (bytes.len() - 1 - 0usize);
                    let offset: usize = 0usize;
                    let mask: u8 = 0b11111111;
                    ((bytes[idx] >> offset) as u8) & mask
                }
            }
            pub struct receive_data_read_R<'a>(&'a R);
            impl receive_data_read_R<'_> {
                pub const fn get_value(self) -> u8 {
                    let mut bytes = (self.0).0;
                    let idx: usize = (bytes.len() - 1 - 0usize);
                    let offset: usize = 0usize;
                    let mask: u8 = 0b11111111;
                    ((bytes[idx] >> offset) as u8) & mask
                }
            }
            pub struct W(pub(super) [u32; 1]);
            impl Default for W {
                fn default() -> Self {
                    unsafe {
                        Self(Default::default())
                            .ls_8_bit_baudrate()
                            .set_value(0)
                            .transmit_data_write()
                            .set_value(0)
                    }
                }
            }
            impl W {
                /// Access to the LS 8 bits of the 16-bit baudrate register.
                /// (Only If bit 7 of the line control register (DLAB bit) is set)
                pub const fn ls_8_bit_baudrate(self) -> LS_8_bit_baudrate_W {
                    LS_8_bit_baudrate_W(self)
                }
                /// Data written is put in the transmit FIFO (Provided it is not full)
                pub const fn transmit_data_write(self) -> transmit_data_write_W {
                    transmit_data_write_W(self)
                }
            }
            pub struct LS_8_bit_baudrate_W(W);
            impl LS_8_bit_baudrate_W {
                pub const unsafe fn set_value(self, value: u8) -> W {
                    let mut bytes = (self.0).0;
                    let idx: usize = (bytes.len() - 1 - 0usize);
                    let offset: usize = 0usize;
                    let mask: u8 = 0b11111111;
                    bytes[idx] |= ((value & mask) as u32) << offset;
                    W(bytes)
                }
            }
            pub struct transmit_data_write_W(W);
            impl transmit_data_write_W {
                pub const unsafe fn set_value(self, value: u8) -> W {
                    let mut bytes = (self.0).0;
                    let idx: usize = (bytes.len() - 1 - 0usize);
                    let offset: usize = 0usize;
                    let mask: u8 = 0b11111111;
                    bytes[idx] |= ((value & mask) as u32) << offset;
                    W(bytes)
                }
            }
        }
        impl super::Peripheral for MU_IO_REG {
            type R = mu_io_reg::R;
            type W = mu_io_reg::W;
            unsafe fn read(base_address: core::ptr::NonNull<()>) -> Self::R {
                use core::num::NonZeroUsize;
                let bytes = unsafe {
                    core::ptr::read_volatile(
                        base_address
                            .map_addr(|addr| NonZeroUsize::new(addr.get() + 0x21_5040).unwrap())
                            .cast()
                            .as_ptr(),
                    )
                };
                mu_io_reg::R(bytes)
            }
            unsafe fn write(base_address: core::ptr::NonNull<()>, value: Self::W) {
                use core::num::NonZeroUsize;
                let bytes = value.0;
                unsafe {
                    core::ptr::write_volatile(
                        base_address
                            .map_addr(|addr| NonZeroUsize::new(addr.get() + 0x21_5040).unwrap())
                            .cast()
                            .as_ptr(),
                        bytes,
                    );
                }
            }
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
}
