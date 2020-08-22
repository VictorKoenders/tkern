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

use crate::memory::PhysicalAddress;
use alloc::vec::Vec;
use apic::ProcessorLocalApic;

mod allocator;
mod apic;
mod dsdt;
mod fadt;
mod header;
mod hpet;
mod rsdt;

pub use self::{
    allocator::TableAllocator,
    apic::{Apic, Madt},
    dsdt::Dsdt,
    fadt::V1 as FadtV1,
    header::Header,
    hpet::{Count as HpetCount, Hpet},
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
    /// High performance event timers
    ///
    /// https://wiki.osdev.org/HPET
    Hpet(&'a Hpet),
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
            Self::Hpet(hpet) => unsafe { &hpet.header },
            Self::Unknown(h) => &h,
        }
    }

    /// Get the name of the table, or the signature if this is an `Unknown` table
    pub fn name(&self) -> &str {
        match self {
            Self::Root(_) => "RSDT",
            Self::FadtV1(_) => "FADT",
            Self::Madt(_) => "APIC (MADT)",
            Self::Hpet(_) => "HPET",
            Self::Unknown(_) => self.header().signature().unwrap_or("Unknown"),
        }
    }
}

/// Contains references to descriptor tables of the entire system.
pub struct System {
    /// RSDT table
    pub rsdt: &'static RSDT,
    /// FADT table
    pub fadt: &'static FadtV1,
    /// MADT table
    pub madt: &'static Madt,
    /// High precision event timers, if available
    pub hpet: Option<&'static Hpet>,
    /// DSDT blob, contains AML code for drivers
    pub dsdt: &'static [u8],
    /// List of processors available on this system
    pub processors: Vec<&'static ProcessorLocalApic>,

    // Needs to stay allocated for the duration of this system, or the SDT above are unmapped
    #[allow(dead_code)]
    allocator: &'static TableAllocator,
}

impl Drop for System {
    fn drop(&mut self) {
        use alloc::boxed::Box;
        let allocator = unsafe { Box::from_raw(self.allocator as *const _ as *mut TableAllocator) };
        drop(allocator);
    }
}

impl System {
    /// Scan the system for descriptor tables. Will panic if a required table is not found
    ///
    /// # Safety
    ///
    /// The caller must ensure the given address is a valid table. Memory allocation and mapping should also be initialized.
    pub unsafe fn scan(addr: PhysicalAddress) -> System {
        fn inner(addr: PhysicalAddress) -> System {
            use alloc::boxed::Box;

            let mut partial = PartialSystem {
                rsdt: None,
                fadt: None,
                madt: None,
                dsdt: None,
                hpet: None,
                processors: Vec::new(),
                allocator: Box::leak(Box::new(TableAllocator::default())),
            };
            let root = unsafe { partial.allocator.get_table(addr) };
            partial.traverse_table(root);

            vga_println!(
                "Descriptor tables take up {} physical mapping(s)",
                partial.allocator.allocation_count()
            );
            vga_println!(
                "Found {} processors, {} enabled",
                partial.processors.len(),
                partial
                    .processors
                    .iter()
                    .filter(|p| unsafe { p.flags.contains(apic::ProcessorFlags::ENABLED) })
                    .count()
            );
            System {
                rsdt: partial.rsdt.unwrap(),
                fadt: partial.fadt.unwrap(),
                madt: partial.madt.unwrap(),
                dsdt: partial.dsdt.unwrap(),
                hpet: partial.hpet,
                processors: partial.processors,
                allocator: partial.allocator,
            }
        }
        inner(addr)
    }
}

struct PartialSystem<'a> {
    rsdt: Option<&'a RSDT>,
    fadt: Option<&'a FadtV1>,
    madt: Option<&'a Madt>,
    dsdt: Option<&'a [u8]>,
    hpet: Option<&'a Hpet>,
    processors: Vec<&'a ProcessorLocalApic>,

    allocator: &'a TableAllocator,
}

impl<'a> PartialSystem<'a> {
    fn traverse_table(&mut self, table: Table<'a>) {
        match table {
            Table::Root(rsdt) => {
                if self.rsdt.is_none() {
                    vga_println!("RSDT    at {:p}", rsdt);
                } else {
                    vga_println!("[WARN] Second RSDT at {:p}, overwriting", rsdt)
                }
                self.rsdt = Some(rsdt);
                for child in rsdt.entries(self.allocator) {
                    self.traverse_table(child);
                }
            }
            Table::Madt(madt) => {
                if self.madt.is_none() {
                    vga_println!("MADT    at {:p}", madt);
                } else {
                    vga_println!("[WARN] Second MADT at {:p}, overwriting", madt)
                }
                self.madt = Some(madt);
                let processors = madt
                    .interrupt_devices(self.allocator)
                    .filter_map(|d| match d {
                        apic::InterruptDevice::ProcessorLocal(processor) => Some(processor),
                        _ => None,
                    })
                    .collect::<Vec<_>>();

                self.processors = processors;
            }
            Table::FadtV1(fadt_v1) => {
                if self.fadt.is_none() {
                    vga_println!("FADT V1 at {:p}", fadt_v1);
                } else {
                    vga_println!("[WARN] Second FADT V1 at {:p}, overwriting", fadt_v1)
                }
                self.dsdt = Some(fadt_v1.dsdt(self.allocator).read(self.allocator));
                self.fadt = Some(fadt_v1);
            }
            Table::Hpet(hpet) => {
                if self.hpet.is_none() {
                    vga_println!("HPET    at {:p}", hpet);
                } else {
                    vga_println!("[WARN] Second HPET at {:p}, overwriting", hpet)
                }
                self.hpet = Some(hpet);
            }
            Table::Unknown(h) => {
                vga_println!("[WARN] Unknown SDT {:?}", h.signature());
            }
        }
    }
}
