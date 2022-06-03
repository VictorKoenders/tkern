use core::{
    alloc::{AllocError, Layout},
    num::NonZeroUsize,
    ptr::NonNull,
};
use utils::atomic_mutex::AtomicMutex;

struct Allocator {
    start: NonNull<()>,
    length: NonZeroUsize,
    offset: AtomicMutex<usize>,
}

#[global_allocator]
static mut ALLOCATOR: Allocator = Allocator {
    start: NonNull::dangling(),
    length: unsafe { NonZeroUsize::new_unchecked(4) },
    offset: AtomicMutex::new(0),
};

impl Allocator {
    pub unsafe fn init(&mut self, addr: NonNull<()>, length: NonZeroUsize) {
        self.start = addr.map_addr(|addr| align(addr, 16));
        self.length = length;
        self.offset = AtomicMutex::new(0);

        *unsafe { self.start.cast().as_mut() } = Header::default();
    }

    unsafe fn find_free_header<'a>(
        &self,
        offset: &mut usize,
        length: usize,
    ) -> Option<&'a mut Header> {
        let start = *offset;

        loop {
            // Get the header at the current offset
            let header = unsafe {
                self.start
                    .map_addr(|a| NonZeroUsize::new_unchecked(a.get() + *offset))
                    .cast::<Header>()
                    .as_mut()
            };
            // Get the offset of the next header
            *offset += if header.total_length == 0 {
                length
            } else {
                header.total_length as usize
            };

            // If the offset is past our max length, set the offset to 0
            if *offset > self.length.get() {
                *offset = 0;
            }

            // If the flag is not occupied, and either the length is 0 or the requested allocation fits in this header, return this header
            if !(header.flags.contains(HeaderFlags::OCCUPIED)
                || (header.total_length != 0 && header.total_length < length as u32))
            {
                break Some(header);
            }
            // else: get the next header

            // We looped around and didn't find an empty spot
            if *offset == start {
                break None;
            }
            // The next header is initialized or the we're about to wrap around the memory, do nothing
            if header.flags.contains(HeaderFlags::NEXT_INITIALIZED)
                || *offset < header as *mut _ as usize
            {
                continue;
            }

            // The next header is not initialized. Mark it as initialized in this header and clear out the next header.
            header.flags.insert(HeaderFlags::NEXT_INITIALIZED);

            let next_header = unsafe {
                self.start
                    .map_addr(|a| NonZeroUsize::new_unchecked(a.get() + *offset))
                    .cast::<Header>()
                    .as_mut()
            };
            *next_header = Header::default();
            break Some(next_header);
        }
    }
}

unsafe impl core::alloc::Allocator for Allocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let mut offset = self.offset.lock();

        let (prefix, length) = Header::get_prefix_and_length(layout);

        let header = unsafe { self.find_free_header(&mut *offset, length) }.ok_or(AllocError)?;

        header.flags.insert(HeaderFlags::OCCUPIED);
        header.update(layout, prefix, length);
        Ok(header.data())
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
        let header = unsafe { Header::from_ptr(ptr) };
        header.flags.remove(HeaderFlags::OCCUPIED);
    }
}

unsafe impl core::alloc::GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match core::alloc::Allocator::allocate(self, layout) {
            Ok(ptr) => ptr.cast().as_ptr(),
            Err(_e) => core::ptr::null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { core::alloc::Allocator::deallocate(self, NonNull::new_unchecked(ptr), layout) };
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        match core::alloc::Allocator::allocate_zeroed(self, layout) {
            Ok(ptr) => ptr.cast().as_ptr(),
            Err(_e) => core::ptr::null_mut(),
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let new_layout = Layout::from_size_align(new_size, layout.align()).unwrap();
        let result = if new_size > layout.size() {
            unsafe {
                core::alloc::Allocator::grow(self, NonNull::new_unchecked(ptr), layout, new_layout)
            }
        } else {
            unsafe {
                core::alloc::Allocator::shrink(
                    &self,
                    NonNull::new_unchecked(ptr),
                    layout,
                    new_layout,
                )
            }
        };
        match result {
            Ok(ptr) => ptr.cast().as_ptr(),
            Err(_e) => core::ptr::null_mut(),
        }
    }
}

#[repr(C, align(16))]
#[derive(Clone, Debug)]
struct Header {
    total_length: u32,
    data_length: u32,
    flags: HeaderFlags,
    prefix_len: u8,
    _padding: [u8; 3],
}

impl Default for Header {
    fn default() -> Self {
        Self {
            total_length: 0,
            data_length: 0,
            flags: HeaderFlags::empty(),
            prefix_len: 0,
            _padding: [0u8; 3],
        }
    }
}

#[test]
fn test_header_size() {
    assert_eq!(core::mem::size_of::<Header>(), 16);
}

fn align(addr: NonZeroUsize, alignment: usize) -> NonZeroUsize {
    let rem = addr.get() % alignment;
    if rem == 0 {
        addr
    } else {
        NonZeroUsize::new(addr.get() + (alignment - rem)).unwrap()
    }
}

impl Header {
    unsafe fn from_ptr<'a>(ptr: NonNull<u8>) -> &'a mut Self {
        unsafe {
            ptr.map_addr(|a| NonZeroUsize::new_unchecked(a.get() - core::mem::size_of::<Header>()))
                .cast()
                .as_mut()
        }
    }

    pub fn update(&mut self, layout: Layout, prefix: u8, length: usize) {
        self.data_length = layout.size() as u32;
        self.total_length = length as u32;
        self.prefix_len = prefix;
    }

    pub fn data(&mut self) -> NonNull<[u8]> {
        let ptr = self as *mut _;
        unsafe {
            let ptr: NonNull<u8> = NonNull::new_unchecked(ptr)
                .map_addr(|a| {
                    NonZeroUsize::new_unchecked(
                        a.get() + core::mem::size_of::<Header>() + self.prefix_len as usize,
                    )
                })
                .cast();
            NonNull::slice_from_raw_parts(ptr, self.data_length as usize)
        }
    }

    pub(crate) fn get_prefix_and_length(layout: Layout) -> (u8, usize) {
        let header_len = core::mem::size_of::<Header>();
        let mut prefix: u8 = 0;
        let mut len = header_len + layout.size();

        // If the end of the header length is not properly aligned, add a prefix
        if header_len % layout.align() != 0 {
            prefix = (layout.align() - (header_len % layout.align())) as u8;
            debug_assert!((header_len + prefix as usize) % layout.align() == 0);
            len += prefix as usize;
        }
        // If the end of the header + prefix + len is not 16-byte aligned, add a suffix
        if len % 16 != 0 {
            len += 16 - (len % 16);
            debug_assert!(len % 16 == 0);
        }
        (prefix, len)
    }
}

bitflags::bitflags! {
    struct HeaderFlags: u32 {
        const OCCUPIED = 0x0000_0001;
        const NEXT_INITIALIZED = 0x0000_0002;
    }
}

pub unsafe fn init(memory_start: NonNull<()>, size: NonZeroUsize) {
    unsafe { ALLOCATOR.init(memory_start, size) }
}
