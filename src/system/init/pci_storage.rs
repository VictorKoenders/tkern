use super::pci::Device;
use crate::system::storage::{Error, Storage};

pub struct PciStorage {
    #[allow(dead_code)] // not implemented yet
    pci: Device,
}

impl PciStorage {
    pub fn new(pci: Device) -> Self {
        Self { pci }
    }
}

impl Storage for PciStorage {
    fn device_count(&self) -> Result<u8, Error> {
        Ok(1)
    }

    fn size(&self, _index: u8) -> Result<usize, Error> {
        Ok(0)
    }

    fn read(&self, _index: u8, _offset: usize, _buffer: &mut [u8]) -> Result<usize, Error> {
        Ok(0)
    }

    fn write(&self, _index: u8, _offset: usize, _buffer: &[u8]) -> Result<usize, Error> {
        Ok(0)
    }
}
