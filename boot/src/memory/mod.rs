#![allow(dead_code)] // this module is being replaced by `memory_new`

use paging::ActivePageTable;

pub mod area_frame_allocator;
mod paging;

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    fn containing_address(address: PhysicalAddress) -> Frame {
        Frame {
            number: address.0 / PAGE_SIZE,
        }
    }

    fn start_address(&self) -> PhysicalAddress {
        PhysicalAddress(self.number * PAGE_SIZE)
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct PhysicalAddress(usize);

impl PhysicalAddress {
    pub fn new(address: usize) -> Self {
        Self(address)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct VirtualAddress(usize);

impl VirtualAddress {
    pub fn new(address: usize) -> Self {
        Self(address)
    }
}

#[derive(Copy, Clone)]
pub struct Page {
    number: usize,
}

impl Page {
    fn containing_address(address: VirtualAddress) -> Page {
        let address = address.0;
        assert!(
            address < 0x0000_8000_0000_0000 || address >= 0xffff_8000_0000_0000,
            "invalid address: 0x{:x}",
            address
        );
        Page {
            number: address / PAGE_SIZE,
        }
    }
    fn start_address(&self) -> VirtualAddress {
        VirtualAddress(self.number * PAGE_SIZE)
    }
    fn end_address(&self) -> VirtualAddress {
        VirtualAddress((self.number + 1) * PAGE_SIZE)
    }
    fn p4_index(&self) -> usize {
        (self.number >> 27) & 0o777
    }
    fn p3_index(&self) -> usize {
        (self.number >> 18) & 0o777
    }
    fn p2_index(&self) -> usize {
        (self.number >> 9) & 0o777
    }
    fn p1_index(&self) -> usize {
        (self.number >> 0) & 0o777
    }
}

pub fn test_paging<A>(allocator: &mut A)
where
    A: FrameAllocator,
{
    let mut page_table = unsafe { ActivePageTable::new() };

    // test it
    let addr = VirtualAddress(42 * 512 * 512 * 4096); // 42th P3 entry
    let page = Page::containing_address(addr);
    let frame = allocator.allocate_frame().expect("no more frames");
    vga_println!(
        "None = {:?}, map to {:?}",
        page_table.translate(addr),
        frame
    );
    page_table.map_to(page, frame, paging::EntryFlags::empty(), allocator);
    vga_println!("Some = {:?}", page_table.translate(addr));
    vga_println!("next free frame: {:?}", allocator.allocate_frame());
    vga_println!(
        "Mapped at {:?} - {:?}, requested {:?}",
        page.start_address(),
        page.end_address(),
        addr
    );

    page_table.unmap(Page::containing_address(addr), allocator);
    vga_println!("None = {:?}", page_table.translate(addr));
}
