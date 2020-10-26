//! System module
//!
//! This module serves as an abstraction of the hardware.
//! The goal is to have a single interface where the underlying implementations do not matter.
//!
//! e.g. instead of trying to figure out what system the hardware uses to store information on disk,
//! this module allows you to request `system.fs().read_exact(idx, offset, &mut buffer)`

pub mod init;
pub mod storage;
pub mod memory;

mod descriptor;

pub(self) use self::descriptor::Descriptor;
use self::storage::SystemStorage;
use self::memory::{PhysicalAddress, Memory};

/// Reference to the hardware system that the kernel is running on
#[allow(dead_code)]
pub struct System {
    pub(self) descriptor: Descriptor,
    pub(self) memory: Memory,
    pub(self) storage: SystemStorage,
}

impl Drop for System {
    fn drop(&mut self) {
        panic!("System should never be dropped");
    }
}

impl System {
    /// Create a new instance of the hardware system
    ///
    /// # Safety
    ///
    /// - The system must ensure that this gets only called once
    /// - The `descriptor_address` must be a valid RSDT address
    pub unsafe fn new(descriptor_address: PhysicalAddress) -> Self {
        let memory = Memory::init();
        let descriptor = Descriptor::scan(descriptor_address).expect("Could not scan descriptor tables");

        let mut system = Self {
            descriptor,
            memory,
            storage: SystemStorage::new(),
        };
        init::init(&mut system);
        system
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    /// Get a reference to the persistent storage of this system
    pub fn storage(&self) -> &SystemStorage {
        &self.storage
    }
}
