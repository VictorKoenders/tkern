#![allow(dead_code)]

use super::{
    allocator::TableAllocator,
    apic::{Madt, ProcessorLocalApic},
    fadt::Fadt,
    header::Header,
    hpet::Hpet,
    mcfg::Mcfg,
    rsdt::RSDT,
};
use crate::VirtualAddress;
use alloc::vec::Vec;

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
    Fadt(Fadt<'a>),
    Mcfg(&'a Mcfg),
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
            Self::Root(r) => &r.header,
            Self::Fadt(fadt) => fadt.header(),
            Self::Madt(madt) => &madt.header,
            Self::Hpet(hpet) => &hpet.header,
            Self::Mcfg(mcfg) => &mcfg.header,
            Self::Unknown(h) => &h,
        }
    }

    pub fn mem_size(&self) -> usize {
        use core::mem::size_of;
        match self {
            Self::Root(_) => size_of::<RSDT>(),
            Self::Fadt(fadt) => fadt.mem_size(),
            Self::Madt(_) => size_of::<Madt>(),
            Self::Hpet(_) => size_of::<Hpet>(),
            Self::Mcfg(_) => size_of::<Mcfg>(),
            Self::Unknown(_) => self.header().length as usize,
        }
    }

    /// Get the name of the table, or the signature if this is an `Unknown` table
    pub fn name(&self) -> &str {
        match self {
            Self::Root(_) => "RSDT",
            Self::Fadt(_) => "FADT",
            Self::Madt(_) => "APIC (MADT)",
            Self::Hpet(_) => "HPET",
            Self::Mcfg(_) => "MCFG",
            Self::Unknown(_) => self.header().signature().unwrap_or("Unknown"),
        }
    }

    pub fn has_trailing_bytes(&self) -> bool {
        matches!(self, Self::Root(_) | Self::Madt(_) | Self::Mcfg(_))
    }
}
pub struct PartialDescriptor<'a> {
    pub rsdt: Option<&'a RSDT>,
    pub fadt: Option<Fadt<'a>>,
    pub madt: Option<&'a Madt>,
    pub dsdt: Option<&'a [u8]>,
    pub hpet: Option<&'a Hpet>,
    pub mcfg: Option<&'a Mcfg>,
    pub processors: Vec<&'a ProcessorLocalApic>,

    pub allocator: &'a TableAllocator,
}

impl<'a> PartialDescriptor<'a> {
    pub fn new(allocator: &'a TableAllocator) -> Self {
        Self {
            rsdt: None,
            fadt: None,
            madt: None,
            dsdt: None,
            hpet: None,
            mcfg: None,
            processors: Vec::new(),
            allocator,
        }
    }

    pub fn traverse_table(&mut self, table: Table<'a>) {
        match table {
            Table::Root(rsdt) => {
                if self.rsdt.is_none() {
                    vga_println!("RSDT    at {:?}", VirtualAddress::from_ref(rsdt));
                } else {
                    vga_println!(
                        "[WARN] Second RSDT at {:?}, overwriting",
                        VirtualAddress::from_ref(rsdt)
                    );
                }
                self.rsdt = Some(rsdt);
                for child in rsdt.entries(self.allocator) {
                    self.traverse_table(child);
                }
            }
            Table::Madt(madt) => {
                if self.madt.is_none() {
                    vga_println!("MADT    at {:?}", VirtualAddress::from_ref(madt));
                } else {
                    vga_println!(
                        "[WARN] Second MADT at {:?}, overwriting",
                        VirtualAddress::from_ref(madt)
                    );
                }
                self.madt = Some(madt);
                let processors = madt
                    .interrupt_devices(self.allocator)
                    .filter_map(|d| match d {
                        super::apic::InterruptDevice::ProcessorLocal(processor) => Some(processor),
                        _ => None,
                    })
                    .collect::<Vec<_>>();

                self.processors = processors;
            }
            Table::Fadt(fadt) => {
                if self.fadt.is_none() {
                    vga_println!("FADT    at {:?}", fadt.virtual_address());
                } else {
                    vga_println!(
                        "[WARN] Second FADT at {:?}, overwriting",
                        fadt.virtual_address()
                    );
                }
                self.dsdt = Some(fadt.dsdt(self.allocator).read(self.allocator));
                self.fadt = Some(fadt);
            }
            Table::Hpet(hpet) => {
                if self.hpet.is_none() {
                    vga_println!("HPET    at {:?}", VirtualAddress::from_ref(hpet));
                } else {
                    vga_println!(
                        "[WARN] Second HPET at {:?}, overwriting",
                        VirtualAddress::from_ref(hpet)
                    );
                }
                self.hpet = Some(hpet);
            }
            Table::Mcfg(mcfg) => {
                if self.mcfg.is_none() {
                    vga_println!("MCFG    at {:?}", VirtualAddress::from_ref(mcfg));
                } else {
                    vga_println!(
                        "[WARN] Second MCFG at {:?}, overwriting",
                        VirtualAddress::from_ref(mcfg)
                    );
                }
                self.mcfg = Some(mcfg);
            }
            Table::Unknown(h) => {
                vga_println!("[WARN] Unknown SDT {:?}", h.signature());
            }
        }
    }
}
