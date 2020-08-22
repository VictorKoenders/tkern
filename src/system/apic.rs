use super::{Header, TableAllocator};
use crate::{memory::PhysicalAddress, utils::slice::SliceUtils};

#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct Madt {
    pub header: Header,
    pub apic_address: u32,
    pub flags: u32,
}

impl Madt {
    pub fn apic<'a>(&self, allocator: &'a TableAllocator) -> &'a Apic {
        let addr =
            allocator.get_mapping_for_physical_address(PhysicalAddress(self.apic_address as u64));
        unsafe { addr.deref_leak() }
    }

    pub fn interrupt_devices(&self, allocator: &TableAllocator) -> InterruptDeviceIterator {
        allocator.ensure_loaded(unsafe { self.header.virtual_address_range() });
        let end_of_madt = self as *const Madt as usize + core::mem::size_of::<Madt>();
        let remaining_length = self.header.length as usize - core::mem::size_of::<Madt>();
        let slice =
            unsafe { core::slice::from_raw_parts(end_of_madt as *const u8, remaining_length) };
        InterruptDeviceIterator { slice }
    }
}

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

#[derive(Debug)]
pub enum InterruptDevice<'a> {
    ProcessorLocal(&'a ProcessorLocalApic),
    Io(&'a IoApic),
    InterruptSourceOverride(&'a InterruptSourceOverride),
    NonMaskableInterrupts(&'a NonMaskableInterrupts),
    Unknown { device_type: u8, data: &'a [u8] },
}

#[repr(packed)]
#[derive(Debug)]
pub struct ProcessorLocalApic {
    processor_id: u8,
    apic_id: u8,
    flags: u32,
}

#[repr(packed)]
#[derive(Debug)]
pub struct IoApic {
    id: u8,
    reserved: u8,
    address: u32,
    global_system_interrupt_base: u32,
}

#[repr(packed)]
#[derive(Debug)]
pub struct InterruptSourceOverride {
    bus_source: u8,
    irq_source: u8,
    global_system_interrupt: u32,
    flags: u16,
}
#[repr(packed)]
#[derive(Debug)]
pub struct NonMaskableInterrupts {
    processor_id: u8,
    flags: u16,
    lint: u8,
}

#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct Apic {
    pub header: Header,
}
