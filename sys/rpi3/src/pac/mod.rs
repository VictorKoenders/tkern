#![allow(
    clippy::upper_case_acronyms,
    clippy::struct_excessive_bools,
    dead_code,
    non_snake_case,
    non_camel_case_types
)]

use core::marker::PhantomData;
use core::ptr::NonNull;

pub mod AUX {
    //! There are two Auxiliary registers which control all three devices. One is the interrupt status
    //! register, the second is the Auxiliary enable register. The Auxiliary IRQ status register can
    //! help to hierarchically determine the source of an interrupt.

    use super::Reg;
    use core::marker::PhantomData;
    use core::ptr::NonNull;

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
        pub Enable_receive_interrupts: bool,
    }

    /// The `AUX_MU_IIR_REG` register shows the interrupt status.
    /// It also has two FIFO enable status bits and (when writing) FIFO clear bits.
    #[derive(sys_derive::Peripheral)]
    #[peripheral(address = 0x21_5048)]
    pub struct MU_IIR_REG {
        /// On read this register shows the interrupt ID bit
        /// - `0b00`: No interrupts
        /// - `0b01`: Transmit holding register empty
        /// - `0b10`: Receiveer holds valid byte
        /// - `0b11`: <Not possible>
        ///
        /// On write:
        /// - Writing with `0b01` will clear the receive FIFO
        /// - Writing with `0b10` will clear the transmit FIFO
        #[field(bits = 2:1, readable, writable)]
        pub interrupt_id: u8,

        /// This bit is clear whenever an interrupt is pending
        #[field(bit = 0, readable, writable, reset = 1)]
        pub interrupt_pending: bool,
    }

    /// The `AUX_MU_LCR_REG` register controls the line data format and gives access to the baudrate register
    #[derive(sys_derive::Peripheral)]
    #[peripheral(address = 0x21_504C)]
    pub struct MU_LCR_REG {
        /// If set the first to Mini UART register give access the the Baudrate register.
        /// During operation this bit must be cleared
        #[field(bit = 7, readable, writable)]
        pub DLAB_access: bool,

        /// If set high the UART1_TX line is pulled low continuously.
        /// If held for at least 12 bits times that will indicate a break condition.
        #[field(bit = 6, readable, writable)]
        pub _Break: bool,

        /// - `0b00` : the UART works in 7-bit mode
        /// - `0b11` : the UART works in 8-bit mod
        #[field(bits = 1:0, readable, writable, unsafe_enum(u8))]
        pub data_size: UartDataSize,
    }

    #[derive(num_enum::UnsafeFromPrimitive, num_enum::IntoPrimitive)]
    #[repr(u8)]
    pub enum UartDataSize {
        _7Bit = 0b00,
        _8Bit = 0b11,
    }

    /// The `AUX_MU_MCR_REG` register controls the 'modem' signals.
    #[derive(sys_derive::Peripheral)]
    #[peripheral(address = 0x21_5050)]
    pub struct MU_MCR_REG {
        /// If clear the UART1_RTS line is high
        /// If set the UART1_RTS line is low
        /// This bit is ignored if the RTS is used for auto-flow control. See the Mini Uart Extra Control register description)
        #[field(bit = 1, readable, writable)]
        pub RTS: bool,
    }

    /// The `AUX_MU_LSR_REG` register shows the data status
    #[derive(sys_derive::Peripheral)]
    #[peripheral(address = 0x21_5054)]
    pub struct MU_LSR_REG {
        /// This bit is set if the transmit FIFO is empty and the
        /// transmitter is idle. (Finished shifting out the last bit).
        #[field(bit = 6, readable, reset = 1)]
        pub Transmitter_idle: bool,

        /// This bit is set if the transmit FIFO can accept at least one byte.
        #[field(bit = 5, readable)]
        pub Transmitter_empty: bool,

        /// This bit is set if there was a receiver overrun.
        /// That is: one or more characters arrived whilst the receive FIFO was full.
        /// The newly arrived charters have been discarded.
        /// This bit is cleared each time this register is read.
        /// To do a non-destructive read of this overrun bit use the Mini Uart Extra Status register
        #[field(bit = 1, readable)]
        pub Receiver_Overrun: bool,

        /// This bit is set if the receive FIFO holds at least 1 symbol.
        #[field(bit = 0, readable)]
        pub Data_ready: bool,
    }

    /// The `AUX_MU_MSR_REG` register shows the 'modem' status.
    #[derive(sys_derive::Peripheral)]
    #[peripheral(address = 0x21_5058)]
    pub struct MU_MSR_REG {
        /// This bit is the inverse of the UART1_CTS input Thus:
        /// - If set the UART1_CTS pin is low
        /// - If clear the UART1_CTS pin is high
        #[field(bit = 5, readable, reset = 1)]
        pub CTS_status: bool,
    }

    /// The `AUX_MU_SCRATCH` is a single byte storage
    #[derive(sys_derive::Peripheral)]
    #[peripheral(address = 0x21_505C)]
    pub struct MU_SCRATCH {
        /// One whole byte extra on top of the 134217728 provided by the SDC
        #[field(bits = 7:0, readable, writable)]
        pub Scratch: u8,
    }

    /// The `AUX_MU_CNTL_REG` provides access to some extra useful and nice features not found on a normal 16550 UART
    #[derive(sys_derive::Peripheral)]
    #[peripheral(address = 0x21_5060)]
    pub struct MU_CNTL_REG {
        /// This bit allows one to invert the CTS auto flow operation polarity.
        /// - If set the CTS auto flow assert level is low*
        /// - If clear the CTS auto flow assert level is high*
        #[field(bit = 7, rw)]
        pub CTS_assert_level: bool,

        /// This bit allows one to invert the RTS auto flow operation polarity.
        /// - If set the RTS auto flow assert level is low*
        /// - If clear the RTS auto flow assert level is high*
        #[field(bit = 6, rw)]
        pub RTS_assert_level: bool,

        /// These two bits specify at what receiver FIFO level the RTS line is de-asserted in auto-flow mode.
        /// - `00` : De-assert RTS when the receive FIFO has 3 empty spaces left.
        /// - `01` : De-assert RTS when the receive FIFO has 2 empty spaces left.
        /// - `10` : De-assert RTS when the receive FIFO has 1 empty space left.
        /// - `11` : De-assert RTS when the receive FIFO has 4 empty spaces left
        #[field(bits = 5:4, rw)]
        pub RTS_AUTO_flow_level: u8,

        /// If this bit is set the transmitter will stop if the CTS line is de-asserted.
        /// If this bit is clear the transmitter will ignore the status of the CTS line
        #[field(bit = 3, rw)]
        pub Enable_transmit_Auto_flow_control_using_CTS: bool,

        /// If this bit is set the RTS line will de-assert if the receive FIFO reaches it 'auto flow' level. In fact the
        /// RTS line will behave as an RTR (Ready To Receive) line.
        /// If this bit is clear the RTS line is controlled by the AUX_MU_MCR_REG register bit 1
        #[field(bit = 2, rw)]
        pub Enable_receive_Auto_flow_control_using_RTS: bool,

        /// If this bit is set the mini UART transmitter is enabled.
        /// If this bit is clear the mini UART transmitter is disabled
        #[field(bit = 1, rw)]
        pub Transmitter_enable: bool,

        /// If this bit is set the mini UART receiver is enabled.
        /// If this bit is clear the mini UART receiver is disabled
        #[field(bit = 0, rw)]
        pub Receiver_enable: bool,
    }

    /// The `AUX_MU_BAUD` register allows direct access to the 16-bit wide baudrate counter.
    #[derive(sys_derive::Peripheral)]
    #[peripheral(address = 0x21_5068)]
    pub struct MU_BAUD_REG {
        /// mini UART baudrate counter
        #[field(bits = 15:0, rw, try_enum(u16))]
        pub Baudrate: BaudRate,
    }

    // The mini UART uses 8-times oversampling. The Baudrate can be calculated from:
    // baudrate = (system_clock_freq) / (8 * (baudrate_reg + 1))
    // If the system clock is 250 MHz and the baud register is zero the baudrate is 31.25 Mega
    // baud. (25 Mbits/sec or 3.125 Mbytes/sec). The lowest baudrate with a 250 MHz system
    // clock is 476 Baud

    // To calculate the enum values: (250000000 / <baud rate>) / 8 - 1

    #[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
    #[repr(u16)]
    pub enum BaudRate {
        _9600 = 3254,
        _19200 = 1627,
        _38400 = 813,
        _57600 = 542,
        _115200 = 270,
    }

    pub struct Peripheral {
        pub IRQ: Reg<IRQ>,
        pub ENB: Reg<ENB>,
        pub MU_IO_REG: Reg<MU_IO_REG>,
        pub MU_IER_REG: Reg<MU_IER_REG>,
        pub MU_IIR_REG: Reg<MU_IIR_REG>,
        pub MU_LCR_REG: Reg<MU_LCR_REG>,
        pub MU_MCR_REG: Reg<MU_MCR_REG>,
        pub MU_LSR_REG: Reg<MU_LSR_REG>,
        pub MU_MSR_REG: Reg<MU_MSR_REG>,
        pub MU_SCRATCH: Reg<MU_SCRATCH>,
        pub MU_CNTL_REG: Reg<MU_CNTL_REG>,
        pub MU_BAUD_REG: Reg<MU_BAUD_REG>,
    }

    impl Peripheral {
        pub unsafe fn new(base_addr: NonNull<()>) -> Self {
            Self {
                IRQ: Reg {
                    base_addr,
                    pd: PhantomData,
                },
                ENB: Reg {
                    base_addr,
                    pd: PhantomData,
                },
                MU_IO_REG: Reg {
                    base_addr,
                    pd: PhantomData,
                },
                MU_IER_REG: Reg {
                    base_addr,
                    pd: PhantomData,
                },
                MU_IIR_REG: Reg {
                    base_addr,
                    pd: PhantomData,
                },
                MU_LCR_REG: Reg {
                    base_addr,
                    pd: PhantomData,
                },
                MU_MCR_REG: Reg {
                    base_addr,
                    pd: PhantomData,
                },
                MU_LSR_REG: Reg {
                    base_addr,
                    pd: PhantomData,
                },
                MU_MSR_REG: Reg {
                    base_addr,
                    pd: PhantomData,
                },
                MU_SCRATCH: Reg {
                    base_addr,
                    pd: PhantomData,
                },
                MU_CNTL_REG: Reg {
                    base_addr,
                    pd: PhantomData,
                },
                MU_BAUD_REG: Reg {
                    base_addr,
                    pd: PhantomData,
                },
            }
        }
    }
}

pub mod GPIO {
    /// The function select registers are used to define the operation of the general-purpose I/O
    /// pins. Each of the 54 GPIO pins has at least two alternative functions as defined in section
    /// 16.2. The FSEL{n} field determines the functionality of the nth GPIO pin. All unused
    /// alternative function lines are tied to ground and will output a “0” if selected. All pins reset
    /// to normal GPIO input operation
    #[derive(sys_derive::Peripheral)]
    #[peripheral(address = 0x20_0004)]
    pub struct GPFSEL1 {
        /// FSEL19 - Function Select 19
        /// - 000 = GPIO Pin 19 is an input
        /// - 001 = GPIO Pin 19 is an output
        /// - 100 = GPIO Pin 19 takes alternate function 0
        /// - 101 = GPIO Pin 19 takes alternate function 1
        /// - 110 = GPIO Pin 19 takes alternate function 2
        /// - 111 = GPIO Pin 19 takes alternate function 3
        /// - 011 = GPIO Pin 19 takes alternate function 4
        /// - 010 = GPIO Pin 19 takes alternate function 5
        #[field(bits = 29:27, rw, unsafe_enum(u8))]
        pub FSEL19: FSEL,
        #[field(bits = 26:24, rw, unsafe_enum(u8))]
        pub FSEL18: FSEL,
        #[field(bits = 23:21, rw, unsafe_enum(u8))]
        pub FSEL17: FSEL,
        #[field(bits = 20:18, rw, unsafe_enum(u8))]
        pub FSEL16: FSEL,
        #[field(bits = 17:15, rw, unsafe_enum(u8))]
        pub FSEL15: FSEL,
        #[field(bits = 14:12, rw, unsafe_enum(u8))]
        pub FSEL14: FSEL,
        #[field(bits = 11:9, rw, unsafe_enum(u8))]
        pub FSEL13: FSEL,
        #[field(bits = 8:6, rw, unsafe_enum(u8))]
        pub FSEL12: FSEL,
        #[field(bits = 5:3, rw, unsafe_enum(u8))]
        pub FSEL11: FSEL,
        #[field(bits = 2:0, rw, unsafe_enum(u8))]
        pub FSEL10: FSEL,
    }

    #[derive(num_enum::UnsafeFromPrimitive, num_enum::IntoPrimitive)]
    #[repr(u8)]
    pub enum FSEL {
        Input = 0b000,
        Output = 0b001,
        Alt0 = 0b100,
        Alt1 = 0b101,
        Alt2 = 0b110,
        Alt3 = 0b111,
        Alt4 = 0b011,
        Alt5 = 0b010,
    }

    /// The GPIO Pull-up/down Register controls the actuation of the internal pull-up/down
    /// control line to ALL the GPIO pins. This register must be used in conjunction with the 2
    /// `GPPUDCLKn` registers.
    ///
    /// Note that it is not possible to read back the current Pull-up/down settings and so it is the
    /// users’ responsibility to ‘remember’ which pull-up/downs are active. The reason for this is
    /// that GPIO pull-ups are maintained even in power-down mode when the core is off, when
    /// all register contents are lost.
    ///
    /// The Alternate function table also has the pull state which is applied after a power down
    #[derive(sys_derive::Peripheral)]
    #[peripheral(address = 0x20_0094)]
    pub struct GPPUD {
        /// PUD - GPIO Pin Pull-up/down
        /// - 00 = Off – disable pull-up/down
        /// - 01 = Enable Pull Down control
        /// - 10 = Enable Pull Up control
        /// - 11 = Reserved
        /// *Use in conjunction with GPPUDCLK0/1/2
        #[field(bits = 1:0, rw)]
        pub PUD: u8,
    }

    /// The GPIO Pull-up/down Clock Registers control the actuation of internal pull-downs on
    /// the respective GPIO pins. These registers must be used in conjunction with the GPPUD
    /// register to effect GPIO Pull-up/down changes. The following sequence of events is
    /// required:
    /// 1. Write to GPPUD to set the required control signal (i.e. Pull-up or Pull-Down or neither
    /// to remove the current Pull-up/down)
    /// 2. Wait 150 cycles – this provides the required set-up time for the control signal
    /// 3. Write to GPPUDCLK0/1 to clock the control signal into the GPIO pads you wish to
    ///    modify – NOTE only the pads which receive a clock will be modified, all others will
    ///    retain their previous state.
    /// 4. Wait 150 cycles – this provides the required hold time for the control signal
    /// 5. Write to GPPUD to remove the control signal
    /// 6. Write to GPPUDCLK0/1 to remove the clock
    #[derive(sys_derive::Peripheral)]
    #[peripheral(address = 0x20_0098)]
    pub struct GPPUDCLK0 {
        /// - 0 = No Effect
        /// - 1 = Assert Clock on line (n)
        /// *Must be used in conjunction with GPPUD
        #[field(bit = 31, rw)]
        pub PUDCLK31: bool,
        #[field(bit = 14, rw)]
        pub PUDCLK14: bool,
        #[field(bit = 15, rw)]
        pub PUDCLK15: bool,
    }

    pub struct Peripheral {
        pub GPFSEL1: super::Reg<GPFSEL1>,
        pub GPPUD: super::Reg<GPPUD>,
        pub GPPUDCLK0: super::Reg<GPPUDCLK0>,
    }

    impl Peripheral {
        pub unsafe fn new(base_addr: core::ptr::NonNull<()>) -> Self {
            Self {
                GPFSEL1: super::Reg {
                    base_addr,
                    pd: core::marker::PhantomData,
                },
                GPPUD: super::Reg {
                    base_addr,
                    pd: core::marker::PhantomData,
                },
                GPPUDCLK0: super::Reg {
                    base_addr,
                    pd: core::marker::PhantomData,
                },
            }
        }
    }
}
pub struct Reg<P: Peripheral> {
    base_addr: NonNull<()>,
    pd: PhantomData<P>,
}

impl<P: Peripheral> Reg<P> {
    pub fn read(&self) -> P::R {
        unsafe { P::read(self.base_addr) }
    }

    pub fn write<F>(&mut self, f: F)
    where
        F: FnOnce(P::W) -> P::W,
    {
        let w = P::W::default();
        let w = f(w);
        unsafe {
            P::write(self.base_addr, w);
        }
    }

    pub fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(P::R, P::W) -> P::W,
    {
        let read = self.read();
        self.write(|w| f(read, w));
    }

    pub fn clear(&mut self) {
        self.write(|w| w);
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
