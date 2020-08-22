use super::{Header, TableAllocator};
use crate::{memory::PhysicalAddress, utils::slice::SliceUtils};

/// MADT describes all of the interrupt controllers in the system.
///
/// https://wiki.osdev.org/MADT
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct Madt {
    /// Common header
    pub header: Header,
    apic_address: u32,
    flags: u32,
}

impl Madt {
    /// Load the local APIC struct
    pub fn apic<'a>(&self, allocator: &'a TableAllocator) -> &'a Apic {
        let addr =
            allocator.get_mapping_for_physical_address(PhysicalAddress(self.apic_address as u64));
        unsafe { addr.deref_leak() }
    }

    /// Iterates over the devices in the MADT table
    pub fn interrupt_devices(&self, allocator: &TableAllocator) -> InterruptDeviceIterator {
        allocator.ensure_loaded(unsafe { self.header.virtual_address_range() });
        let end_of_madt = self as *const Madt as usize + core::mem::size_of::<Madt>();
        let remaining_length = self.header.length as usize - core::mem::size_of::<Madt>();
        let slice =
            unsafe { core::slice::from_raw_parts(end_of_madt as *const u8, remaining_length) };
        InterruptDeviceIterator { slice }
    }
}

/// Iterator that iterates over the devices in the MADT table
pub struct InterruptDeviceIterator<'a> {
    slice: &'a [u8],
}

impl<'a> Iterator for InterruptDeviceIterator<'a> {
    type Item = InterruptDevice<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let device_type = self.slice.pop_front()?;
        let length = self.slice.pop_front()? as usize - 2; // the first 2 bytes are `device_type` and `length`
        if length > self.slice.len() {
            panic!(
                "MADT entry was longer than the MADT table ({} > {})",
                length,
                self.slice.len()
            );
        }
        let payload_ptr = self.slice.as_ptr();
        self.slice = &self.slice[length..];

        match device_type {
            0x00 => {
                debug_assert_eq!(length, core::mem::size_of::<ProcessorLocalApic>());
                Some(InterruptDevice::ProcessorLocal(unsafe {
                    &*(payload_ptr as *const _)
                }))
            }
            0x01 => {
                debug_assert_eq!(length, core::mem::size_of::<IoApic>());
                Some(InterruptDevice::Io(unsafe { &*(payload_ptr as *const _) }))
            }
            0x02 => {
                debug_assert_eq!(length, core::mem::size_of::<InterruptSourceOverride>());
                Some(InterruptDevice::InterruptSourceOverride(unsafe {
                    &*(payload_ptr as *const _)
                }))
            }
            0x04 => {
                debug_assert_eq!(length, core::mem::size_of::<NonMaskableInterrupts>());
                Some(InterruptDevice::NonMaskableInterrupts(unsafe {
                    &*(payload_ptr as *const _)
                }))
            }
            _ => {
                let payload = unsafe { core::slice::from_raw_parts(payload_ptr, length) };
                Some(InterruptDevice::Unknown {
                    device_type,
                    data: payload,
                })
            }
        }
    }
}

/// Collection of APIC entries that can exist in the MADT table.
#[derive(Debug)]
pub enum InterruptDevice<'a> {
    /// Information about the available processor
    ProcessorLocal(&'a ProcessorLocalApic),
    /// Information about the available IO
    Io(&'a IoApic),
    /// Information about the available overrides for interrupt sources
    InterruptSourceOverride(&'a InterruptSourceOverride),
    /// Information about the non-maskable interrupts (NMI)
    NonMaskableInterrupts(&'a NonMaskableInterrupts),
    Unknown {
        device_type: u8,
        data: &'a [u8],
    },
}

/// Contains information about the available CPUs
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct ProcessorLocalApic {
    /// The ID of the processor
    pub processor_id: u8,
    apic_id: u8,

    /// The flags of the processor
    pub flags: ProcessorFlags,
}

/// Contains information about device IO
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct IoApic {
    id: u8,
    reserved: u8,
    address: u32,
    global_system_interrupt_base: u32,
}

/// Contains information about overrides for interrupt sources
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct InterruptSourceOverride {
    bus_source: u8,
    irq_source: u8,
    global_system_interrupt: u32,
    flags: u16,
}

/// Contains information about NonMaskable Interrupts (NMI)
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct NonMaskableInterrupts {
    processor_id: u8,
    flags: u16,
    lint: u8,
}

/// Local APIC struct. Not implemented yet.
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct Apic {
    /// Common SDT header
    pub header: Header,
}

bitflags::bitflags! {
    /// CPU flags
    pub struct ProcessorFlags : u32 {
        /// The CPU is enabled
        const ENABLED = 1<<0;
        /// The CPU can be enabled
        const CAN_BE_ENABLED = 1<<1;
    }
}
