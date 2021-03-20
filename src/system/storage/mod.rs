pub mod atapi;

use alloc::sync::Arc;
use smallvec::SmallVec;

/// Abstraction of the persistent storage available on this system.
/// This will most likely be the hard drive that is present in the device.
pub struct SystemStorage {
    drivers: SmallVec<[Arc<dyn Storage>; 10]>,
}

#[allow(unused_variables)]
impl SystemStorage {
    pub(super) fn new() -> SystemStorage {
        Self {
            drivers: SmallVec::new(),
        }
    }

    pub fn register(&mut self, storage: impl Storage + 'static) {
        self.drivers.push(Arc::new(storage));
    }

    /// Return the amount of drives that are available on this system.
    /// This value may be cached for performance reasons.
    ///
    /// This may report more or less drives than there are storage devices in the system.
    /// Drivers are allowed to merge or split up devices based on what the driver deems best.
    ///
    /// Even though this function is marked async, some implementations may infact be executed in a synchronous matter.
    pub async fn drive_count(&self) -> Result<u8, Error> {
        let mut result = 0;
        for driver in &self.drivers {
            result += driver.device_count()?;
        }
        Ok(result)
    }

    /// Return the storage size available on the given drive.
    /// This value may be cached for performance reasons
    ///
    /// Even though this function is marked async, some implementations may infact be executed in a synchronous matter.
    pub async fn storage_size(&self, mut index: u8) -> Result<usize, Error> {
        for driver in &self.drivers {
            let device_count = driver.device_count()?;
            if device_count > index {
                return driver.size(index);
            }
            index -= device_count;
        }
        Err(Error::NoValidDriver)
    }

    /// Read N bytes from the given disk `index` at the given `offset`.
    /// If the buffer could not be filled, an error is thrown.
    /// The contents of `buffer` can be partially overwritten before an error is thrown.
    ///
    /// Even though this function is marked async, some implementations may infact be executed in a synchronous matter.
    pub async fn read_exact(
        &self,
        mut index: u8,
        offset: usize,
        buffer: &mut [u8],
    ) -> Result<(), Error> {
        for driver in &self.drivers {
            let device_count = driver.device_count()?;
            if device_count > index {
                let n = driver.read(index, offset, buffer)?;
                return if n == buffer.len() {
                    Ok(())
                } else {
                    Err(Error::WouldBlock)
                };
            }
            index -= device_count;
        }
        Err(Error::NoValidDriver)
    }

    /// Write N bytes to the given disk `index` at the given `offset`.
    /// If the full buffer could not be written, an error is thrown.
    /// It is possible for the write action to partially succeed, while still throwing an error.
    ///
    /// Even though this function is marked async, some implementations may infact be executed in a synchronous matter.
    pub async fn write_exact(
        &self,
        mut index: u8,
        offset: usize,
        buffer: &[u8],
    ) -> Result<(), Error> {
        for driver in &self.drivers {
            let device_count = driver.device_count()?;
            if device_count > index {
                let n = driver.write(index, offset, buffer)?;
                return if n == buffer.len() {
                    Ok(())
                } else {
                    Err(Error::WouldBlock)
                };
            }
            index -= device_count;
        }
        Err(Error::NoValidDriver)
    }
}

pub enum Error {
    NoValidDriver,
    WouldBlock,
}

pub trait Storage {
    fn device_count(&self) -> Result<u8, Error>;
    fn size(&self, index: u8) -> Result<usize, Error>;
    fn read(&self, index: u8, offset: usize, buffer: &mut [u8]) -> Result<usize, Error>;
    fn write(&self, index: u8, offset: usize, buffer: &[u8]) -> Result<usize, Error>;
}
