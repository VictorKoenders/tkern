use super::{Header, TableAllocator};
use crate::memory::{PhysicalAddress, VirtualAddress, PAGE_SIZE};

#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct Dsdt {
    pub header: Header,
}

impl Dsdt {
    pub fn read<'a>(&self, allocator: &'a TableAllocator) -> &'a [u8] {
        let len = unsafe { self.header.bytes_len_after_header() };
        let ptr = unsafe { self.header.ptr_after_header() };
        let self_addr = VirtualAddress(ptr as usize as u64);

        let (mut physical, end_of_mapping) = allocator
            .with_mapping_for_address(self_addr, |mapping| {
                (mapping.physical_address(), mapping.last_virtual_address())
            });

        let self_mapping_len = (end_of_mapping.0 - self_addr.0) as usize;

        let mut index = self_mapping_len;
        while index < len {
            physical = PhysicalAddress(physical.0 + PAGE_SIZE);
            allocator.get_mapping_for_physical_address(physical);
            index += PAGE_SIZE as usize;
        }

        unsafe { core::slice::from_raw_parts(ptr, len) }
    }
}
