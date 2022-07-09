use core::fmt;
pub use core::time::Duration;

pub trait Time {
    fn now(&self) -> Instant;
    /// Spin for at least `duration` time. This may spin for longer.
    ///
    /// # Errors
    ///
    /// May return a [`SpinError`] if spinning failed for some reason.
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
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_duration().fmt(fmt)
    }
}

impl Instant {
    #[must_use]
    pub fn from_nanos(nanos: u64) -> Self {
        Self { nanos }
    }

    #[must_use]
    pub fn as_duration(self) -> Duration {
        Duration::from_nanos(self.nanos)
    }
}

#[derive(Debug)]
pub struct SpinError(());
