use super::Header;
use core::marker::PhantomData;

#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct Mcfg {
    /// Common SDT header
    pub header: Header,

    _reserved: u64,
}

impl Mcfg {
    pub fn entries(&self) -> McfgEntryIterator {
        let len_of_self = core::mem::size_of::<Mcfg>();
        McfgEntryIterator {
            ptr: ((&self as *const _ as usize) + len_of_self) as *const u8,
            length: self.header.length - len_of_self as u32,
            pd: PhantomData,
        }
    }
}

pub struct McfgEntryIterator<'a> {
    ptr: *const u8,
    length: u32,
    pd: PhantomData<&'a ()>,
}

impl<'a> Iterator for McfgEntryIterator<'a> {
    type Item = &'a McfgEntry;
    fn next(&mut self) -> Option<&'a McfgEntry> {
        if self.length == 0 {
            return None;
        }
        let entry = unsafe { &*(self.ptr as *const McfgEntry) };
        let entry_mem_size = core::mem::size_of::<McfgEntry>() as u32;
        if self.length < entry_mem_size {
            self.length = 0;
        } else {
            self.length -= entry_mem_size;
        }
        Some(entry)
    }
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct McfgEntry {
    pub base_address: u64,
    pub pci_segment_group_number: u16,
    pub start_pci_bus_number: u8,
    pub end_pci_bus_number: u8,
    _reserved: u32,
}

impl core::fmt::Debug for McfgEntry {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_struct("McfgEntry")
            .field("base_address", {
                &format_args!("0x{:08X}", { self.base_address })
            })
            .field("pci_segment_group_number", &{
                self.pci_segment_group_number
            })
            .field("start_pci_bus_number", &self.start_pci_bus_number)
            .field("end_pci_bus_number", &self.end_pci_bus_number)
            .finish()
    }
}
