use super::Location;

/// Base Address Registers (or BARs) can be used to hold memory addresses used by the device, or offsets for port addresses.
/// Typically, memory address BARs need to be located in physical ram while I/O space BARs can reside at any memory address
/// (even beyond physical memory).
#[derive(Debug)]
pub enum BaseAddress {
    /// Address is in the memory space
    Memory {
        /// The 16-byte aligned base address
        base_address: u64,
        /// When a base address register is marked as Prefetchable,
        /// it means that the region does not have read side effects (reading from that memory range doesn't change any state),
        /// and it is allowed for the CPU to cache loads from that memory region and read it in bursts (typically cache line sized).
        /// Hardware is also allowed to merge repeated stores to the same address into one store of the latest value.
        prefetchable: bool,
        /// The Type field of the Memory Space BAR Layout specifies the size of the base register and where in memory it can be mapped.
        memory_type: MemoryType,
    },
    /// Address is in the IO space
    Io {
        /// The 4-byte aligned base address of this address
        address: u32,
    },
}

impl BaseAddress {
    pub(super) fn read(location: Location, next_location: Location) -> Option<Self> {
        let val = super::read_location_u32(location);
        // https://wiki.osdev.org/PCI#Base_Address_Registers
        // lowest bit indicates if this is memory or IO

        if val & 1 == 0 {
            // memory
            let memory_type = if val == 0x10 {
                MemoryType::Bits64
            } else {
                MemoryType::Bits32
            };
            let base_address = match memory_type {
                MemoryType::Bits64 => {
                    let next = super::read_location_u32(next_location);
                    ((val as u64) & 0xFFFFFFF0) + (((next as u64) & 0xFFFFFFFF) << 32)
                }
                MemoryType::Bits32 => (val & 0xFFFFFFF0) as u64,
            };

            Some(BaseAddress::Memory {
                base_address,
                prefetchable: (val & 0b0100) > 0,
                memory_type,
            })
        } else {
            // IO
            Some(BaseAddress::Io {
                address: val & 0xFFFFFFFC,
            })
        }
    }

    /// Returns true if the base address is considered 0.
    pub fn is_null(&self) -> bool {
        match self {
            BaseAddress::Memory { base_address, .. } => *base_address == 0,
            BaseAddress::Io { address, .. } => *address == 0,
        }
    }
}

/// Determines where the memory BAR can be mapped
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemoryType {
    /// the base register is 32-bits wide and can be mapped anywhere in the 32-bit Memory Space.
    Bits32,
    /// the base register is 64-bits wide and can be mapped anywhere in the 64-bit Memory Space (A 64-bit base address register consumes 2 of the base address registers available).
    Bits64,
}
