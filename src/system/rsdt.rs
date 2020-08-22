use super::{Header, Table, TableAllocator};
use crate::memory::PhysicalAddress;

/// The root system descriptor table. This is used to be a list of child tables.
///
/// https://wiki.osdev.org/RSDT
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct RSDT {
    /// The common header of all system descriptor tables
    pub header: Header,
}

fn _test_size() {
    unsafe {
        core::mem::transmute::<[u8; 36], RSDT>([0u8; 36]);
    }
}

impl RSDT {
    /// Get the child entries of this root descriptor table
    pub fn entries<'a>(
        &'a self,
        allocator: &'a TableAllocator,
    ) -> impl Iterator<Item = Table> + 'a {
        let entries = unsafe {
            let ptr = self.header.ptr_after_header() as *const u32;
            let len = self.header.bytes_len_after_header() / 4;
            core::slice::from_raw_parts(ptr, len)
        };
        entries
            .iter()
            .map(move |phys| unsafe { allocator.get_table(PhysicalAddress(*phys as u64)) })
    }
}
