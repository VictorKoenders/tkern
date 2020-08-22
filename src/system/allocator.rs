use super::*;
use crate::memory::{
    AllocateOptions, AllocatedPhysicalPageMapping, Mapper, PhysicalAddress, VirtualAddress,
    PAGE_SIZE,
};
use alloc::vec::Vec;
use core::cell::RefCell;

/// Allocate one or multiple [AllocatedPhysicalPageMapping], and keep track of it
///
/// The mappings, and the data read from it, are valid as long as this allocator is valid.
/// When this allocator gets dropped, all the mappings get unmapped.
#[derive(Default)]
pub struct TableAllocator {
    allocs: RefCell<Vec<AllocatedPhysicalPageMapping>>,
}

impl TableAllocator {
    /// Get a table from the given address.
    /// This allocator will ensure the memory is mapped and will stay mapped.
    ///
    /// # Safety
    ///
    /// The given address must be a valid System Descriptor Table.
    pub unsafe fn get_table(&self, address: PhysicalAddress) -> Table {
        let addr = self.get_mapping_for_physical_address(address);
        let header = addr.deref_leak::<Header>();
        match &header.signature {
            b"RSDT" => Table::Root(addr.deref_leak()),
            b"FACP" if header.length == FadtV1::LENGTH => Table::FadtV1(addr.deref_leak()),
            b"APIC" => Table::Madt(addr.deref_leak::<Madt>()),
            //b"HPET" => Table::HighPerformanceEvent(addr.deref_leak::<HPET>()),
            _ => Table::Unknown(header),
        }
    }

    pub(super) unsafe fn get_table_known_type<T>(&self, address: PhysicalAddress) -> &T {
        let addr = self.get_mapping_for_physical_address(address);
        addr.deref_leak::<T>()
    }

    /// Get the mapping that is associated with the given virtual address.
    ///
    /// Will panic if the given address is not mapped.
    pub fn with_mapping_for_address<T>(
        &self,
        address: VirtualAddress,
        cb: impl FnOnce(&AllocatedPhysicalPageMapping) -> T,
    ) -> T {
        let inner = self.allocs.borrow();
        if let Some(addr) = inner.iter().find(|a| a.contains(address)) {
            cb(addr)
        } else {
            unreachable!()
        }
    }

    /// Get the virtual address for the given physical address
    pub fn get_mapping_for_physical_address(&self, address: PhysicalAddress) -> VirtualAddress {
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

    /// Get the amount of allocations that this allocator has done
    pub fn allocation_count(&self) -> usize {
        self.allocs.borrow().len()
    }

    /// Ensure the given range of virtual addresses is loaded
    pub fn ensure_loaded(&self, range: core::ops::Range<VirtualAddress>) {
        let allocs = self.allocs.borrow_mut();

        let start = range.start;
        let end = range.end;
        let mapping = allocs.iter().find(|a| a.contains(start)).unwrap();
        let mut addr = mapping.last_virtual_address();
        while addr.0 < end.0 {
            if allocs.iter().find(|a| a.contains(addr)).is_none() {
                let offset = addr.0 - mapping.virtual_address().0;
                self.get_mapping_for_physical_address(PhysicalAddress(
                    mapping.physical_address().0 + offset,
                ));
            }
            addr.0 += PAGE_SIZE;
        }
    }
}
