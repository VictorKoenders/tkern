use super::{PhysicalAddress, VirtualAddress};
use bitflags::bitflags;
use core::{
    marker::{PhantomData, Sized},
    ops::{Index, IndexMut},
    ptr::NonNull,
};

bitflags! {
    pub struct EntryFlags: u64 {
        // Used by the MMU
        const PRESENT =         1 << 0;
        const WRITABLE =        1 << 1;
        const USER_ACCESSIBLE = 1 << 2;
        const WRITE_THROUGH =   1 << 3;
        const NO_CACHE =        1 << 4;
        const ACCESSED =        1 << 5;
        const DIRTY =           1 << 6;
        const HUGE_PAGE =       1 << 7;
        const GLOBAL =          1 << 8;
        const NO_EXECUTE =      1 << 63;

        // Freely usable by the kernel
        // 9-11, 52-62
        const ALLOCATED =       1 << 9;
    }
}
pub struct TableEntry(u64);

impl TableEntry {
    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }

    pub fn flags(&self) -> EntryFlags {
        EntryFlags::from_bits_truncate(self.0)
    }
    pub fn set(&mut self, address: PhysicalAddress, flags: EntryFlags) {
        let address = address.0;
        assert!(
            address & !0x000fffff_fffff000 == 0,
            "Allocated at an invalid address: 0x{:X}",
            address
        );
        self.0 = (address as u64) | flags.bits() | EntryFlags::PRESENT.bits();
    }

    pub fn deallocate(&mut self) {
        if self.flags().contains(EntryFlags::ALLOCATED) {
            let ptr = self.0 & 0x000fffff_fffff000;
            unsafe {
                alloc::alloc::dealloc(
                    ptr as *mut _,
                    core::alloc::Layout::from_size_align(TABLE_PAGE_SIZE, TABLE_PAGE_SIZE).unwrap(),
                )
            }
        }
        self.0 = 0;
    }

    pub fn create(&mut self, flags: EntryFlags) {
        if self.flags().contains(EntryFlags::PRESENT) {
            vga_println!("Warning: Trying to allocate a paging table entry that is already allocated. Ignoring.");
            return;
        }
        let ptr = unsafe {
            alloc::alloc::alloc_zeroed(
                core::alloc::Layout::from_size_align(TABLE_PAGE_SIZE, TABLE_PAGE_SIZE).unwrap(),
            )
        };
        vga_println!("Allocated frame at {:p} with flags {:?}", ptr, flags);
        self.set(
            PhysicalAddress(ptr as *mut _ as u64),
            flags | EntryFlags::ALLOCATED,
        );
    }
}

const ENTRY_COUNT: usize = 512;
pub(super) const TABLE_PAGE_SIZE: usize = 4096;

fn _size_check() {
    unsafe {
        core::mem::transmute::<[u8; TABLE_PAGE_SIZE], Table<Level1>>([0u8; TABLE_PAGE_SIZE]);
    }
}

pub struct Table<L> {
    entries: [TableEntry; ENTRY_COUNT],
    level: PhantomData<L>,
}

impl<L> Index<usize> for Table<L> {
    type Output = TableEntry;

    fn index(&self, index: usize) -> &TableEntry {
        &self.entries[index]
    }
}

impl<L> IndexMut<usize> for Table<L> {
    fn index_mut(&mut self, index: usize) -> &mut TableEntry {
        &mut self.entries[index]
    }
}

const P4: *mut Table<Level4> = 0xffffffff_fffff000 as *mut _;

pub trait TableLevel: Sized + 'static {}

pub enum Level4 {}
pub enum Level3 {}
pub enum Level2 {}
pub enum Level1 {}

impl TableLevel for Level4 {}
impl TableLevel for Level3 {}
impl TableLevel for Level2 {}
impl TableLevel for Level1 {}

pub trait HierarchicalLevel: TableLevel {
    type NextLevel: TableLevel + 'static;
}

impl HierarchicalLevel for Level4 {
    type NextLevel = Level3;
}

impl HierarchicalLevel for Level3 {
    type NextLevel = Level2;
}

impl HierarchicalLevel for Level2 {
    type NextLevel = Level1;
}

pub struct ActivePageTable {
    p4: NonNull<Table<Level4>>,
}

// TODO: this is not actually safe, but we don't have multithreading yet
unsafe impl Send for ActivePageTable {}

impl<L: HierarchicalLevel> Table<L> {
    pub fn next_table(&self, index: usize) -> Option<&Table<L::NextLevel>> {
        self.next_table_address(index)
            .map(|address| unsafe { &*(address as *const _) })
    }

    pub fn next_table_mut(&mut self, index: usize) -> Option<&mut Table<L::NextLevel>> {
        self.next_table_address(index)
            .map(|address| unsafe { &mut *(address as *mut _) })
    }

    fn next_table_address(&self, index: usize) -> Option<usize> {
        let entry_flags = self[index].flags();
        if entry_flags.contains(EntryFlags::PRESENT) && !entry_flags.contains(EntryFlags::HUGE_PAGE)
        {
            let table_address = self as *const _ as usize;
            Some((table_address << 9) | (index << 12))
        } else {
            None
        }
    }

    pub fn next_table_create(&mut self, index: usize) -> &mut Table<L::NextLevel> {
        if self.next_table(index).is_none() {
            assert!(
                !self.entries[index].flags().contains(EntryFlags::HUGE_PAGE),
                "mapping code does not support huge pages:\n{}[{}] with flags {:?}",
                core::any::type_name::<L>(),
                index,
                self.entries[index].flags()
            );
            self.entries[index].create(EntryFlags::WRITABLE);
        }
        self.next_table_mut(index).unwrap()
    }
}

impl ActivePageTable {
    pub unsafe fn new() -> ActivePageTable {
        ActivePageTable {
            p4: NonNull::new_unchecked(P4),
        }
    }

    #[allow(dead_code)]
    fn p4(&self) -> &Table<Level4> {
        unsafe { self.p4.as_ref() }
    }

    fn p4_mut(&mut self) -> &mut Table<Level4> {
        unsafe { self.p4.as_mut() }
    }

    pub fn allocate_virtual_address(
        &mut self,
        physical: PhysicalAddress,
        virt: VirtualAddress,
        options: super::AllocateOptions,
    ) {
        let p4 = self.p4_mut();
        let p3 = p4.next_table_create(virt.p4_index());
        let p2 = p3.next_table_create(virt.p3_index());
        let p1 = p2.next_table_create(virt.p2_index());

        assert!(p1[virt.p1_index()].is_unused());
        p1[virt.p1_index()].set(physical, options.flags());
    }

    pub fn clear(&mut self, virt: VirtualAddress) {
        let entry = self
            .p4_mut()
            .next_table_mut(virt.p4_index())
            .and_then(|p3| p3.next_table_mut(virt.p3_index()))
            .and_then(|p2| p2.next_table_mut(virt.p2_index()));
        let entry = match entry {
            Some(e) => e,
            None => return,
        };
        entry[virt.p1_index()].deallocate();
    }
}
