mod command;

pub use self::command::Command;

use crate::pac;
use core::{num::NonZeroU32, time::Duration};
use embedded_time::rate::{Hertz, Kilohertz, Megahertz};

pub const FREQ_SETUP: Kilohertz = Kilohertz(400);
pub const FREQ_NORMAL: Megahertz = Megahertz(25);
pub const BASE_CLOCK: Megahertz = Megahertz(50);

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
/// SD card types
pub enum SdCardType {
    Unknown,
    MMC,
    Type1,
    Type2Sc,
    Type2Hc,
}

impl Default for SdCardType {
    fn default() -> Self {
        Self::Unknown
    }
}

pub struct Emmc<'a, TIME: driver::time::Time> {
    emmc: pac::EMMC,
    time: &'a TIME,

    state: Option<EmmcState>,
}

// TODO: Turns this into bitflags
#[derive(Debug, Copy, Clone)]
pub struct CardStatus(pub u32);

impl CardStatus {
    pub fn is_error(&self) -> bool {
        self.0 & 0xfff9c004 > 0
    }

    pub fn is_st_app_cmd(&self) -> bool {
        self.0 & 0x00000020 > 0
    }

    // TODO: Give this a better name
    pub fn st_app_cmd() -> Self {
        Self(0x00000020)
    }
}

#[allow(dead_code)]
struct EmmcState {
    card_capacity: u64, // Card capacity expanded .. calculated from card details
    emmc_card_type: SdCardType, // Card type
    rca: Option<NonZeroU32>, // Card rca
    status: CardStatus, // Card last status
    last_cmd: Command,  // Last command
}

impl EmmcState {
    fn new(last_cmd: Command) -> Self {
        Self {
            card_capacity: 0,
            emmc_card_type: SdCardType::Unknown,
            rca: None,
            status: CardStatus(0),
            last_cmd,
        }
    }
}

impl<'a, TIME> Emmc<'a, TIME>
where
    TIME: driver::time::Time,
{
    pub fn new(emmc: pac::EMMC, time: &'a TIME) -> Self {
        Self {
            emmc,
            time,
            state: None,
        }
    }

    pub fn reset(&mut self) -> Result {
        self.emmc.control1.write(|w| w.srst_hc().set_bit()); // Reset the complete host circuit
        self.time.spin_for(Duration::from_micros(10))?; // Wait 10 microseconds

        log::info!("EMMC: reset card.");
        let start_time = self.time.now();
        while self.emmc.control1.read().srst_hc().bit_is_set() {
            // Host circuit reset not clear yet
            if self.time.elapsed(start_time).as_micros() > 10 {
                // Timeout reached
                log::info!("EMMC: ERROR: failed to reset.");

                return Err(Error::TimedOut); // Return reset SD Card error
            }
        }

        self.emmc.control1.write(|c| c.srst_data().set_bit()); // reset data lines
        self.time.spin_for(Duration::from_micros(10))?; // Wait 10 microseconds

        self.emmc.control1.reset();

        // Set SD bus power VDD1 to 3.3V at initialization.
        //
        // The RPi 4's controller is more compliant with the standard.
        // This additional step was not needed on the RPi 1-3
        self.emmc.control0.reset();
        self.time.spin_for(Duration::from_micros(1))?;
        self.emmc
            .control0
            .write(|c| c.bus_voltage().v3_3().bus_power().set_bit());
        self.emmc.control1.reset();
        self.time.spin_for(Duration::from_micros(1))?;

        /* Set clock to setup frequency */
        self.set_clock2(FREQ_SETUP)?;

        // Enable interrupts and interrupt mask
        self.emmc.irpt_en.reset();
        self.emmc.irpt_mask.write(|m| m.card().set_bit());
        self.emmc.interrupt.write(|m| unsafe { m.bits(u32::MAX) });
        let interrupt = self.emmc.interrupt.read();
        self.emmc
            .irpt_en
            .modify(|r, w| unsafe { w.bits(r.bits() | interrupt.bits()) });

        /* Reset our card structure entries */
        self.state = None;

        self.send(Command::GoIdleState) // Send GO_IDLE_STATE to card
    }

    pub fn send(&mut self, cmd: Command) -> Result {
        if cmd.requires_app_cmd() {
            if let Err(e) = self.send_app_cmd() {
                self.debug_response(&e);
                return Err(e);
            }
        }
        self.send_command(cmd, None)?;

        if cmd.requires_app_cmd() {
            let state = self.assume_state()?;
            if state.rca.is_none() || !state.status.is_st_app_cmd() {
                // TODO: What is this error?
                return Err(Error::AppCmd);
            }
        }
        Ok(())
    }

    fn debug_response(&self, err: &Error) {
        log::info!(
            target: "EMMC",
            "CMD {:?}, resp: {:?}",
            self.state.as_ref().map(|s| s.last_cmd.name()),
            err,
        );
        log::info!(target: "EMMC",
            "STATUS: 0x{:08X}, CONTROL1: 0x{:08X}, INTERRUPT: 0x{:08X}",
            self.emmc.status.read().bits(),
            self.emmc.control1.read().bits(),
            self.emmc.interrupt.read().bits(),
        );
        log::info!(
            target: "EMMC",
            "RESP3: 0x{:08X}, RESP2: 0x{:08X}, RESP1: 0x{:08X}, RESP0: 0x{:08X}",
            self.emmc.resp3.read().bits(),
            self.emmc.resp2.read().bits(),
            self.emmc.resp1.read().bits(),
            self.emmc.resp0.read().bits()
        );
    }

    fn send_app_cmd(&mut self) -> Result {
        let last_rca = self.state.as_ref().and_then(|s| s.rca);
        self.send_command(Command::AppCmd, last_rca)?;

        if let Some(state) = &self.state {
            if !state.status.is_st_app_cmd() {
                return Err(Error::AppCmdNotAccepted);
            }
        }
        Ok(())
    }

    fn send_command(&mut self, cmd: Command, arg: Option<NonZeroU32>) -> Result {
        let arg = match arg {
            Some(a) => a.get(),
            None => 0,
        };

        self.wait_for_command()?;
        log::info!(target: "EMMC", "Sending command: {:?}", cmd);
        self.set_last_cmd(cmd);

        self.emmc.interrupt.reset();
        self.emmc.arg1.write(|w| unsafe { w.bits(arg) });
        self.emmc.cmdtm.write(|w| cmd.write(w));

        if let Some(delay) = cmd.delay() {
            self.time
                .spin_for(Duration::from_micros(delay.get() as _))?;
        }

        self.wait_for_interrupt(Duration::from_micros(100), |r| r.cmd_done())?;

        let response = self.emmc.resp0.read().bits();

        log::info!(target: "EMMC", "Response: 0x{:08X}", response);
        match cmd.response_size() {
            command::ResponseSize::None => Ok(()),
            command::ResponseSize::_48BitsWithBusy => {
                let state = self.assume_state_mut()?;
                state.status = CardStatus(response);
                if state.status.is_error() {
                    log::info!(target: "EMMC", "CMD_BUSY49BIT_RESP");
                    Err(Error::Status(state.status))
                } else {
                    Ok(())
                }
            }
            command::ResponseSize::_48Bits => todo!(),
            command::ResponseSize::_136Bits => todo!(),
        }
    }

    fn wait_for_interrupt<F, R>(&self, timeout: Duration, f: F) -> Result
    where
        F: Fn(bcm2837_pac::emmc::interrupt::R) -> R,
        R: core::ops::Deref<Target = bcm2837_pac::generic::FieldReader<bool, bool>>,
    {
        let start = self.time.now();
        loop {
            if f(self.emmc.interrupt.read()).bit_is_set() {
                return Ok(());
            }
            if self.time.elapsed(start) > timeout {
                break;
            }
        }

        let interrupt_status = self.emmc.interrupt.read();
        if interrupt_status.cto_err().bit_is_set() || interrupt_status.dto_err().bit_is_set() {
            log::info!(target: "EMMC",
                "Wait for interrupt, STATUS: 0x{:08X}, INTERRUPTS: 0x{:08X}, RESP0: 0x{:08X}, time_diff: {:?}",
                self.emmc.status.read().bits(),
                interrupt_status.bits(),
                self.emmc.resp0.read().bits(),
                self.time.elapsed(start)
            );
            self.clear_all_interrupts();
        } else if interrupt_status.err().bit_is_set() {
            log::info!(target: "EMMC",
                "Error waiting for for interrupt, STATUS: 0x{:08X}, INTERRUPTS: 0x{:08X}, RESP0: 0x{:08X}",
                self.emmc.status.read().bits(),
                interrupt_status.bits(),
                self.emmc.resp0.read().bits(),
            );
            self.clear_all_interrupts();
        }

        Err(Error::TimedOut)
    }

    fn clear_all_interrupts(&self) {
        self.emmc
            .interrupt
            .modify(|r, w| unsafe { w.bits(r.bits()) });
    }

    fn wait_for_command(&mut self) -> Result {
        let start = self.time.now();
        loop {
            if self.emmc.status.read().cmd_inhibit().bit_is_clear() {
                break;
            }
            if self.emmc.interrupt.read().any_error() {
                break;
            }
            if self.time.elapsed(start).as_micros() > 100 {
                return Err(Error::TimedOut);
            }
        }
        Ok(())
    }

    fn assume_state(&self) -> Result<&EmmcState> {
        self.state.as_ref().ok_or(Error::StateNotInitialized)
    }

    fn assume_state_mut(&mut self) -> Result<&mut EmmcState> {
        self.state.as_mut().ok_or(Error::StateNotInitialized)
    }

    fn set_last_cmd(&mut self, last_cmd: Command) {
        if let Some(state) = self.state.as_mut() {
            state.last_cmd = last_cmd;
        } else {
            self.state = Some(EmmcState::new(last_cmd));
        }
    }

    /// Set the SD clock to the given frequency (derived from the base clock)
    ///
    /// # Errors
    ///
    /// - Clock: A fatal error occurred setting the clock
    pub fn set_clock2(
        &self,
        freq: impl TryInto<Hertz, Error = embedded_time::ConversionError>,
    ) -> Result {
        let freq = freq
            .try_into()
            .expect("Invalid frequency given to Emmc::set_clock2");
        // A divisor of zero doesnt work. I think a divisor of 1 equates to half the base clock rate.
        // TODO: need to find confirmation of the above.
        assert!(freq < BASE_CLOCK);

        let freq = freq.0;
        let base_clock: Hertz = BASE_CLOCK.try_into().unwrap();
        let base_clock = base_clock.0;

        let mut div;
        div = base_clock / (freq << 1);
        if (base_clock / (div << 1)) > freq {
            div += 1
        }

        #[rustfmt::skip]
        self.emmc.control1.write(|w| unsafe {
            w.clk_freq8().bits((div & 0xFF) as u8)
             .clk_freq_ms2().bits((div >> 8) as u8)
             .data_tounit().bits(0xf)
             .clk_gensel().clear_bit()
             .clk_en().set_bit()
             .clk_intlen().set_bit()
        });

        /* Wait for clock to be stablized */
        let start = self.time.now();

        while self.emmc.control1.read().clk_stable().bit_is_clear() {
            // Clock not stable yet
            // Timeout not reached
            if self.time.elapsed(start).as_micros() > 100 {
                log::info!("EMMC: ERROR: failed to get stable clock.");

                return Err(Error::Clock); // Return clock error
            }
        }

        log::info!("Divisor = {:?}, Freq Set = {:?}", div, BASE_CLOCK);

        // Clock frequency set worked
        Ok(())
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    TimedOut,
    Clock,
    StateNotInitialized,
    AppCmd,
    AppCmdNotAccepted,
    Status(CardStatus),
    Time(driver::time::SpinError),
}

impl From<driver::time::SpinError> for Error {
    fn from(e: driver::time::SpinError) -> Self {
        Self::Time(e)
    }
}

type Result<T = ()> = core::result::Result<T, Error>;

trait InterruptUtils {
    fn any_error(&self) -> bool;
}

impl InterruptUtils for pac::emmc::interrupt::R {
    fn any_error(&self) -> bool {
        self.acmd_err().bit_is_set()
            || self.dend_err().bit_is_set()
            || self.dcrc_err().bit_is_set()
            || self.dto_err().bit_is_set()
            || self.cbad_err().bit_is_set()
            || self.cend_err().bit_is_set()
            || self.ccrc_err().bit_is_set()
            || self.cto_err().bit_is_set()
            || self.err().bit_is_set()
    }
}
