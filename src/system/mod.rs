//! System descriptors tables
//!
//! This module is used to read the descriptor tables from the system.
//!
//! ```no_run
//! # let rsdt_address = 0;
//! let mapping = TableAllocator::default();
//! let virt = mapping.get_allocator(PhysicalAddress(rsdt_address));
//! let table = unsafe { Table::from_address(virt) };
//! // table is probably a RSDT
//! // tables and their entries are valid as long as TableAllocator exists
//! ```

mod allocator;
mod header;
mod rsdt;

pub use self::{allocator::TableAllocator, header::Header, rsdt::RSDT};

/// A strongly typed representation of the available System Descriptor Tables.
///
/// https://wiki.osdev.org/RSDT#What_can_you_find.3F
pub enum Table<'a> {
    /// The Root System Descriptor Table. Will contain a list of child tables
    Root(&'a RSDT),
    /// An unknown table, contains only the header
    Unknown(&'a Header),
}

impl<'a> Table<'a> {
    /// Get the header of any system descriptor table
    pub fn header(&self) -> &Header {
        match self {
            Self::Root(r) => &r.header,
            Self::Unknown(h) => &h,
        }
    }
}
