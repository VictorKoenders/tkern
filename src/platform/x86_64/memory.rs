use x86_64::{
    registers::control::Cr3,
    structures::paging::{
        OffsetPageTable, Page, PageTable as PageTableInner, PageTableFlags, PhysFrame, Size4KiB,
    },
};

pub mod prelude {
    pub use super::*;
    pub use x86_64::structures::paging::{MapperAllSizes, PhysFrame};
}

pub trait Mapper = x86_64::structures::paging::Mapper<Size4KiB>;
pub trait FrameAllocator = x86_64::structures::paging::FrameAllocator<Size4KiB>;
pub type MapError = x86_64::structures::paging::mapper::MapToError<Size4KiB>;
pub type PageTable = OffsetPageTable<'static>;

pub use x86_64::{PhysAddr, VirtAddr};

/// Initialize a new OffsetPageTable.
///
/// # Safety
///
/// The caller must guarantee that the complete physical memory is mapped
/// to virtual memory at the passed `physical_memory_offset`. Also, this
/// function must be only called once to avoid aliasing `&mut` references
/// (which is undefined behavior).
pub unsafe fn init(physical_memory_offset: VirtAddr) -> PageTable {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTableInner {
    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTableInner = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

pub fn init_heap(
    mapper: &mut impl Mapper,
    frame_allocator: &mut impl FrameAllocator,
    heap_start: usize,
    heap_size: usize,
) -> Result<(), MapError> {
    let page_range = {
        let heap_start = VirtAddr::new(heap_start as u64);
        let heap_end = heap_start + heap_size - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    Ok(())
}

unsafe impl x86_64::structures::paging::FrameAllocator<Size4KiB>
    for crate::allocator::BootInfoFrameAllocator
{
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
