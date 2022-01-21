use super::TimeManager;
use cortex_a::{
    asm::barrier,
    registers::{CNTFRQ_EL0, CNTPCT_EL0, CNTP_CTL_EL0, CNTP_TVAL_EL0},
};
use driver::time;
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};

const NS_PER_S: u64 = 1_000_000_000;

impl TimeManager {
    pub(crate) fn init(&self) {
        self.freq.set(CNTFRQ_EL0.get() as u64);
    }
    pub(crate) fn is_inited(&self) -> bool {
        self.freq.get() != 0
    }

    fn get_nanos(&self) -> u64 {
        // Prevent that the counter is read ahead of time due to out-of-order execution.
        unsafe { barrier::isb(barrier::SY) };
        let ticks = CNTPCT_EL0.get() * NS_PER_S;

        ticks / self.freq.get()
    }
}

impl time::Time for TimeManager {
    fn now(&self) -> time::Instant {
        time::Instant::from_nanos(self.get_nanos())
    }

    fn spin_for(&self, duration: core::time::Duration) -> Result<(), time::SpinError> {
        // Instantly return on zero
        if duration.as_nanos() == 0 {
            return Ok(());
        }

        // Calculate the register compare value.
        let frq = CNTFRQ_EL0.get();
        let x = match frq.checked_mul(duration.as_nanos() as u64) {
            None => {
                return Err(time::SpinError::TooLong);
            }
            Some(val) => val,
        };
        let tval = x / NS_PER_S;

        if tval == 0 {
            // Check if it is within supported bounds.
            return Err(time::SpinError::TooShort);
        } else if tval > u32::max_value().into() {
            // The upper 32 bits of CNTP_TVAL_EL0 are reserved.
            return Err(time::SpinError::TooLong);
        }

        // Set the compare value register.
        CNTP_TVAL_EL0.set(tval);

        // Kick off the counting (ENABLE) and disable timer interrupt (IMASK).
        CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::SET + CNTP_CTL_EL0::IMASK::SET);

        // ISTATUS will be '1' when cval ticks have passed. Busy-check it.
        while !CNTP_CTL_EL0.matches_all(CNTP_CTL_EL0::ISTATUS::SET) {}

        // Disable counting again.
        CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::CLEAR);

        Ok(())
    }
}
