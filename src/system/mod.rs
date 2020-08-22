//! System descriptors tables
//!
//! This module is used to read the descriptor tables from the system.
//!
//! For more information see https://wiki.osdev.org/RSDT#What_can_you_find.3F
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
mod apic;
mod dsdt;
mod fadt;
mod header;
mod rsdt;

pub use self::{
    allocator::TableAllocator,
    apic::{Apic, Madt},
    dsdt::Dsdt,
    fadt::V1 as FadtV1,
    header::Header,
    rsdt::RSDT,
};

/// A strongly typed representation of the available System Descriptor Tables.
///
/// https://wiki.osdev.org/RSDT#What_can_you_find.3F
#[derive(Debug)]
pub enum Table<'a> {
    /// The Root System Descriptor Table. Will contain a list of child tables
    Root(&'a RSDT),
    /// Fixed ACPI Description table
    ///
    /// https://wiki.osdev.org/FADT
    FadtV1(&'a FadtV1),
    /// Multiple APIC description tables, contains
    ///
    /// https://wiki.osdev.org/MADT
    Madt(&'a Madt),
    /// An unknown table, contains only the header
    Unknown(&'a Header),
}

impl<'a> Table<'a> {
    /// Get the header of any system descriptor table
    pub fn header(&self) -> &Header {
        match self {
            Self::Root(r) => unsafe { &r.header },
            Self::FadtV1(fadt) => unsafe { &fadt.header },
            Self::Madt(madt) => unsafe { &madt.header },
            Self::Unknown(h) => &h,
        }
    }

    /// Get the name of the table, or the signature if this is an `Unknown` table
    pub fn name(&self) -> &str {
        match self {
            Self::Root(_) => "RSDT",
            Self::FadtV1(_) => "FADT",
            Self::Madt(_) => "APIC (MADT)",
            Self::Unknown(_) => self.header().signature().unwrap_or("Unknown"),
        }
    }
}
