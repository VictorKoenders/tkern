#![allow(missing_docs, non_snake_case)]

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
mod mcfg;
mod partial;
mod rsdt;

use self::partial::{PartialDescriptor, Table};
pub use self::{
    allocator::TableAllocator,
    apic::{Apic, Madt},
    dsdt::Dsdt,
    fadt::{Fadt, V1 as FadtV1, V2 as FadtV2, V3 as FadtV3},
    header::Header,
    hpet::{Count as HpetCount, Hpet},
    mcfg::Mcfg,
    rsdt::RSDT,
};

/// Contains references to descriptor tables of the entire system.
pub struct Descriptor {
    /// RSDT table
    pub rsdt: &'static RSDT,
    /// FADT table
    pub fadt: Fadt<'static>,
    /// MADT table
    pub madt: &'static Madt,
    /// High precision event timers, if available
    pub hpet: Option<&'static Hpet>,
    pub mcfg: Option<&'static Mcfg>,
    /// DSDT blob, contains AML code for drivers
    pub dsdt: &'static [u8],
    /// List of processors available on this system
    pub processors: Vec<&'static ProcessorLocalApic>,

    // Needs to stay allocated for the duration of this system, or the SDT above are unmapped
    #[allow(dead_code)]
    allocator: &'static TableAllocator,
}

impl Drop for Descriptor {
    fn drop(&mut self) {
        use alloc::boxed::Box;
        let allocator = unsafe { Box::from_raw(self.allocator as *const _ as *mut TableAllocator) };
        drop(allocator);
    }
}

impl Descriptor {
    /// Scan the system for descriptor tables. Will panic if a required table is not found
    ///
    /// # Safety
    ///
    /// The caller must ensure the given address is a valid table. Memory allocation and mapping should also be initialized.
    pub unsafe fn scan(addr: PhysicalAddress) -> Result<Descriptor, ()> {
        use alloc::boxed::Box;

        let mut partial = PartialDescriptor::new(Box::leak(Box::new(TableAllocator::default())));
        let root = partial.allocator.get_table(addr);
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
                .filter(|p| p.flags.contains(apic::ProcessorFlags::ENABLED))
                .count()
        );

        if let Some(mcfg) = &partial.mcfg {
            vga_println!("MCFG: ");
            for entry in mcfg.entries() {
                vga_println!(" - {:?}", entry);
            }
        }
        Ok(Descriptor {
            rsdt: partial.rsdt.unwrap(),
            fadt: partial.fadt.unwrap(),
            madt: partial.madt.unwrap(),
            dsdt: partial.dsdt.unwrap(),
            hpet: partial.hpet,
            mcfg: partial.mcfg,
            processors: partial.processors,
            allocator: partial.allocator,
        })
    }
}
