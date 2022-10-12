pub use core::time::Duration;
use cortex_a::registers::{CNTFRQ_EL0, CNTPCT_EL0};
use tock_registers::interfaces::Readable;

pub struct Time;

impl Time {
    /// Get the duration that this kernel has been running
    pub fn uptime() -> Duration {
        const NS_PER_S: u64 = 1_000_000_000;
        let cntpct = CNTPCT_EL0.get();

        let current_count: u64 = cntpct * NS_PER_S;
        let frq = CNTFRQ_EL0.get();

        Duration::from_nanos(current_count / frq)
    }
}
