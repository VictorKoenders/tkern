#[derive(Debug)]
#[non_exhaustive]
pub enum SpinError {
    TooShort,
    TooLong,
}

use core::fmt;
use core::time::Duration;

pub trait Time {
    fn now(&self) -> Instant;
    fn spin_for(&self, duration: Duration) -> Result<(), SpinError>;
    fn elapsed(&self, old: Instant) -> Duration {
        let now = self.now();

        Duration::from_nanos(now.nanos - old.nanos)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant {
    nanos: u64,
}

impl fmt::Debug for Instant {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.as_duration().fmt(fmt)
    }
}

impl Instant {
    pub fn from_nanos(nanos: u64) -> Self {
        Self { nanos }
    }

    pub fn as_duration(self) -> Duration {
        Duration::from_nanos(self.nanos)
    }
}
