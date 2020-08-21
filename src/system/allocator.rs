use super::*;
use crate::memory::{AllocateOptions, Mapper, PhysicalAddress, VirtualAddress};

/// Allocate one or multiple [AllocatedPhysicalPageMapping], and keep track of it
///
/// The mappings, and the data read from it, are valid as long as this allocator is valid.
/// When this allocator gets dropped, all the mappings get unmapped.
#[derive(Default)]
pub struct TableAllocator {
    allocs: core::cell::RefCell<alloc::vec::Vec<crate::memory::AllocatedPhysicalPageMapping>>,
}

impl TableAllocator {
    /// Get a table from the given address.
    /// This allocator will ensure the memory is mapped and will stay mapped.
    ///
    /// # Safety
    ///
    /// The given address must be a valid System Descriptor Table.
    pub unsafe fn get_table(&self, address: PhysicalAddress) -> Table {
        let mut addr = self.get_allocator(address);
        let mut buffer = [0u8; 4];
        addr.read_slice(&mut buffer);
        match &buffer {
            b"RSDT" => Table::Root(addr.deref_leak::<RSDT>()),
            //b"FACP" => Table::FixedAcpi(addr.deref_leak::<FACP>()),
            //b"APIC" => Table::MultipleApic(addr.deref_leak::<APIC>()),
            //b"HPET" => Table::HighPerformanceEvent(addr.deref_leak::<HPET>()),
            _ => Table::Unknown(addr.deref_leak::<Header>()),
        }
    }

    fn get_allocator(&self, address: PhysicalAddress) -> VirtualAddress {
        let mut borrow = self.allocs.borrow_mut();
        for alloc in borrow.iter() {
            if let Some(virt) = alloc.try_get_virtual_address(address) {
                return virt;
            }
        }
        let alloc =
            Mapper::access_mut(|m| m.map_physical_address(address, AllocateOptions::empty()));
        borrow.push(alloc);
        borrow.last().unwrap().virtual_address()
    }
}
