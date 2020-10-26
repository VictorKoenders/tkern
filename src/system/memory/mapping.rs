//! Memory mapping.
//!
//! See [Mapper] for more information.

use super::paging::{ActivePageTable, EntryFlags};
use crate::{PhysicalAddress, VirtualAddress};
use bitflags::bitflags;
use lazy_static::lazy_static;
use spin::Mutex;

/// MMU page size, 4 KB
pub const PAGE_SIZE: u64 = 4 * 1024; // 4kb

/// An allocated physical map. This can be obtained by calling [Mapper]'s `map_physical_address`.
///
/// When this mapping goes out of scope, it will be deallocated automatically.
#[derive(Debug)]
pub struct AllocatedPhysicalPageMapping {
    // The start of the physical address of this page
    physical_page_start: PhysicalAddress,
    // The start of the virtual address of this page
    virtual_page_start: VirtualAddress,
    // The offset from the base. The user requested a physical address (e.g. 0xB8050)
    // But the page is on 4kb boundaries (e.g. 0xB8000)
    // The offset would be (0xB8050 - 0xB8000) 50
    page_offset: u16, // u16 because the offset can be max 4096
}

impl AllocatedPhysicalPageMapping {
    /// Get the physical address of this mapping
    pub fn physical_address(&self) -> PhysicalAddress {
        self.physical_page_start
    }

    /// Get the last virtual address that is valid for this mapping
    pub fn last_virtual_address(&self) -> VirtualAddress {
        VirtualAddress(self.virtual_page_start.0 + PAGE_SIZE - 1)
    }

    /// Get the virtual address of this mapping
    pub fn virtual_address(&self) -> VirtualAddress {
        VirtualAddress(self.virtual_page_start.0 + self.page_offset as u64)
    }

    /// Returns `true` if this mapping contains the given virtual address
    pub fn contains(&self, address: VirtualAddress) -> bool {
        self.virtual_page_start.0 <= address.0 && self.virtual_page_start.0 + PAGE_SIZE > address.0
    }

    /// Try to get the virtual address of the given physical address.
    /// If the physical address exists in this mapping, the relevant [VirtualAddress] is returned.
    pub fn try_get_virtual_address(&self, address: PhysicalAddress) -> Option<VirtualAddress> {
        if address.0 < self.physical_page_start.0 {
            None
        } else {
            let offset = address.0 - self.physical_page_start.0;
            if offset > PAGE_SIZE {
                None
            } else {
                Some(VirtualAddress(self.virtual_page_start.0 + offset))
            }
        }
    }

    /*
    pub fn get_offset(&self, offset: usize) -> Option<VirtualAddress> {
        // if the user requests offset 0x0040,
        // and the base is 0x5000,
        // and the page_offset is 0x0030
        // then the total offset is (0x30 + 0x40) = 0x70
        // the address returned should be 0x5070

        let total_offset = (self.page_offset as u64) + (offset as u64);
        if total_offset > PAGE_SIZE {
            // address outside of page
            None
        } else {
            Some(VirtualAddress(self.virtual_page_start.0 + total_offset))
        }
    }
    */
}

impl Drop for AllocatedPhysicalPageMapping {
    fn drop(&mut self) {
        Mapper::access_mut(|mapper| mapper.deallocate(self));
    }
}

lazy_static! {
    // TODO: (Dis)prove that a MaybeUninit<Mapper> is safe and faster than an Option<Mapper>
    static ref MAPPER: Mutex<Option<Mapper>> = Mutex::new(None);
}

bitflags! {
    /// Options that will be passed to the given memory map. These are used by [Mapper]'s `map_physical_address` method.
    pub struct AllocateOptions : u64 {
        /// The physical address that is mapped should be able to be written to
        const WRITABLE = 1 << 1;
        /// The physical address that is mapped should be accessible by a non-kernel user
        const USER_ACCESSIBLE = 1 << 2;
    }
}

impl AllocateOptions {
    pub(super) fn flags(self) -> EntryFlags {
        let mut flags = EntryFlags::empty();
        if self.contains(AllocateOptions::WRITABLE) {
            flags |= EntryFlags::WRITABLE;
        }
        if self.contains(AllocateOptions::USER_ACCESSIBLE) {
            flags |= EntryFlags::USER_ACCESSIBLE;
        }
        flags
    }
}

#[derive(Default)]
struct TableUsage([u8; 7]);

impl TableUsage {
    fn get_index_and_mask(index: usize) -> (usize, u8) {
        let remainder = index % 8;
        (index / 8, 1 << (7 - remainder))
    }

    fn is_mapped(&self, index: usize) -> bool {
        let (index, mask) = Self::get_index_and_mask(index);
        (self.0[index] & mask) > 0
    }

    fn set(&mut self, index: usize) {
        let (index, mask) = Self::get_index_and_mask(index);
        self.0[index] |= mask
    }

    fn clear(&mut self, index: usize) {
        let (index, mask) = Self::get_index_and_mask(index);
        self.0[index] &= !mask
    }

    fn find_next_free(&self) -> Option<usize> {
        // TODO(optimize): Get last index that was set, and start there
        for i in 0..512 {
            if !self.is_mapped(i) {
                return Some(i);
            }
        }

        None
    }
}

/// An abstraction around the CPU's MMU.
/// This can be used to map physical addresses to virtual addresses.
///
/// The mapper must be initialized when the kernel starts by calling [init].
/// Afterwards the mapper can be accessed with the [access_mut](#fn.access_mut) method.
pub struct Mapper {
    p1_usage: TableUsage,
    p1_table: ActivePageTable,
}

impl Mapper {
    /// Map a physical address to a random virtual address.
    /// This allocation will automatically be deallocated when the returned [AllocatedPhysicalPageMapping] is dropped.
    ///
    /// When mapping a physical address that you want to write to, make sure to add `AllocateOptions::WRITABLE`.
    pub fn map_physical_address(
        &mut self,
        physical_address: PhysicalAddress,
        options: AllocateOptions,
    ) -> AllocatedPhysicalPageMapping {
        // get the start of the page
        let page_address = (physical_address.0 / PAGE_SIZE) * PAGE_SIZE;
        let page_offset = physical_address.0 - page_address;
        debug_assert!(page_offset < 4096);
        let page_offset = page_offset as u16;

        let physical_page_start = PhysicalAddress(page_address);
        let virtual_page_start = self.find_available_virtual_address();
        self.p1_usage.set(virtual_page_start.p1_index());

        self.p1_table
            .allocate_virtual_address(physical_page_start, virtual_page_start, options);

        AllocatedPhysicalPageMapping {
            physical_page_start,
            virtual_page_start,
            page_offset,
        }
    }

    fn find_available_virtual_address(&mut self) -> VirtualAddress {
        let p4_index = 42;
        let p3_index = 0;
        let p2_index = 0;
        let p1_index = self
            .p1_usage
            .find_next_free()
            .expect("Used all 512 p1 paging entries");

        VirtualAddress::from_paging_indices(p4_index, p3_index, p2_index, p1_index)
    }

    fn deallocate(&mut self, mapping: &AllocatedPhysicalPageMapping) {
        self.p1_usage.clear(mapping.virtual_page_start.p1_index());
        self.p1_table.clear(mapping.virtual_page_start);
        crate::arch::flush_tlb(mapping.virtual_page_start);
    }

    /*
    pub fn access<F, T>(f: F) -> T
    where
        F: FnOnce(&Mapper) -> T,
    {
        crate::arch::without_interrupts(|| f(MAPPER.lock().as_ref().unwrap()))
    }
    */

    /// Access the memory mapping.
    /// This is done from within a callback to ensure that the mapper is only access from a single thread and won't be interrupted.
    pub fn access_mut<F, T>(f: F) -> T
    where
        F: FnOnce(&mut Mapper) -> T,
    {
        crate::arch::interrupts::without_interrupts(|| f(MAPPER.lock().as_mut().unwrap()))
    }
}

/// Initialize the mapper. This should be called exactly once at the start of the kernel execution.
///
/// # Safety
///
/// This must ensure the given active page is valid for the duration of the
/// mapper, and that the mapper isn't already initialized.
///
/// This must also ensure that a valid [ALLOCATOR] is set by calling [allocator::init]
///
/// [ALLOCATOR]: ../allocator/static.ALLOCATOR.html
/// [allocator::init]: ../allocator/fn.init.html
pub unsafe fn init() {
    crate::arch::interrupts::without_interrupts(|| {
        let mut mapper = MAPPER.lock();
        *mapper = Some(Mapper {
            p1_usage: TableUsage::default(),
            p1_table: ActivePageTable::new(),
        });
    });
}
