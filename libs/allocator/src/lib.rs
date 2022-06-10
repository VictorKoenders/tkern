#![cfg_attr(not(test), no_std)]
#![feature(strict_provenance, allocator_api, nonnull_slice_from_raw_parts)]
#![warn(unsafe_op_in_unsafe_fn, clippy::pedantic, rust_2018_idioms)]
#![allow(clippy::cast_possible_truncation)]

use core::{
    alloc::{AllocError, Layout},
    num::NonZeroUsize,
    ptr::NonNull,
};
use utils::atomic_mutex::AtomicMutex;

pub struct Allocator {
    start: NonNull<()>,
    length: NonZeroUsize,
    offset: AtomicMutex<usize>,
}

impl Allocator {
    /// Create a new uninited instance of this allocator
    ///
    /// # Safety
    ///
    /// The caller must ensure that this allocator gets inited properly
    #[must_use]
    pub const unsafe fn new() -> Self {
        Allocator {
            start: NonNull::dangling(),
            length: unsafe { NonZeroUsize::new_unchecked(4) },
            offset: AtomicMutex::new(0),
        }
    }

    /// Initialize the allocator at the given address and with the given length
    ///
    /// # Safety
    ///
    /// The caller must ensure that the memory address is valid for the lifetime of this allocator
    pub unsafe fn init(&mut self, addr: NonNull<()>, length: NonZeroUsize) {
        self.start = addr.map_addr(|addr| align(addr, 16));
        self.length = length;
        self.offset = AtomicMutex::new(0);

        *unsafe { self.start.cast().as_mut() } = Header::default();
    }

    #[cfg(test)]
    fn new_from_slice(slice: &mut [u8]) -> Self {
        unsafe {
            let mut allocator = Self::new();
            allocator.init(
                NonNull::new_unchecked(slice.as_mut_ptr().cast()),
                NonZeroUsize::new(slice.len()).unwrap(),
            );
            allocator
        }
    }

    unsafe fn find_free_header<'a>(
        &self,
        offset: &mut usize,
        length: usize,
    ) -> Option<&'a mut Header> {
        let start = *offset;

        loop {
            if *offset + length > self.length.get() {
                *offset = 0;
                // Make sure we don't loop forever if `length` is larger than `self.length.get()`
                if *offset + length > self.length.get() {
                    return None;
                }
            }

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

            // If the offset is at or past our max length, set the offset to 0
            if *offset >= self.length.get() {
                *offset = 0;
                // Offset 0 is initialized when the allocator is created, so we don't have to check for that
            } else if !header.flags.contains(HeaderFlags::NEXT_INITIALIZED) {
                // The next header is not initialized. Mark it as initialized in this header and clear out the next header.
                header.flags.insert(HeaderFlags::NEXT_INITIALIZED);

                let next_header = unsafe {
                    self.start
                        .map_addr(|a| NonZeroUsize::new_unchecked(a.get() + *offset))
                        .cast::<Header>()
                        .as_mut()
                };
                *next_header = Header::default();
            }

            // If the flag is not occupied, and either the length is 0 or the requested allocation fits in this header, return this header
            if !(header.flags.contains(HeaderFlags::OCCUPIED)
                || (header.total_length != 0 && header.total_length < length as u32))
            {
                break Some(header);
            }

            // We looped around and didn't find an empty spot
            if *offset == start {
                break None;
            }
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

    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        debug_assert!(
            new_layout.size() <= old_layout.size(),
            "`new_layout.size()` must be smaller than or equal to `old_layout.size()`"
        );

        let header = unsafe { Header::from_ptr(ptr) };
        header.data_length = new_layout.size() as u32;

        Ok(header.data())
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
            ptr.map_addr(|a| {
                let addr = a.get() - core::mem::size_of::<Header>();
                let padding = addr % core::mem::align_of::<Header>();
                NonZeroUsize::new_unchecked(addr - padding)
            })
            .cast()
            .as_mut()
        }
    }

    pub fn update(&mut self, layout: Layout, prefix: u8, length: usize) {
        self.data_length = layout.size() as u32;
        self.prefix_len = prefix;
        if self.total_length == 0 {
            self.total_length = length as u32;
        }
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
        let header_layout = Layout::new::<Header>();

        let prefix: u8 = padding_needed_for(header_layout.size(), layout.align()) as u8;
        let with_prefix_length = header_layout.size() + layout.size() + prefix as usize;
        let total_length =
            with_prefix_length + padding_needed_for(with_prefix_length, header_layout.align());

        debug_assert!((header_layout.size() + prefix as usize) % layout.align() == 0);
        debug_assert!(total_length % 16 == 0);
        (prefix, total_length)
    }
}

const fn padding_needed_for(addr: usize, next_align: usize) -> usize {
    // Rounded up value is:
    //   len_rounded_up = (len + align - 1) & !(align - 1);
    // and then we return the padding difference: `len_rounded_up - len`.
    //
    // We use modular arithmetic throughout:
    //
    // 1. align is guaranteed to be > 0, so align - 1 is always
    //    valid.
    //
    // 2. `len + align - 1` can overflow by at most `align - 1`,
    //    so the &-mask with `!(align - 1)` will ensure that in the
    //    case of overflow, `len_rounded_up` will itself be 0.
    //    Thus the returned padding, when added to `len`, yields 0,
    //    which trivially satisfies the alignment `align`.
    //
    // (Of course, attempts to allocate blocks of memory whose
    // size and padding overflow in the above manner should cause
    // the allocator to yield an error anyway.)

    let len_rounded_up =
        addr.wrapping_add(next_align).wrapping_sub(1) & !next_align.wrapping_sub(1);
    len_rounded_up.wrapping_sub(addr)
}

bitflags::bitflags! {
    struct HeaderFlags: u32 {
        const OCCUPIED = 0x0000_0001;
        const NEXT_INITIALIZED = 0x0000_0002;
    }
}

#[test]
fn test_header_size() {
    assert_eq!(core::mem::size_of::<Header>(), 16);
}

#[cfg(test)]
fn with_randomized_memory<const N: usize>(mut buffer: [u8; N], mut cb: impl FnMut(&Allocator)) {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();

    for _ in 0..100 {
        rng.fill(buffer.as_mut_slice());
        let allocator = Allocator::new_from_slice(&mut buffer);
        cb(&allocator);
    }
}

#[test]
fn test_alloc_dealloc() {
    extern crate alloc;
    use alloc::vec::Vec;

    with_randomized_memory([0u8; 100], |allocator| {
        let mut vec = Vec::new_in(&allocator);
        assert!(vec.is_empty());
        vec.push(1u32);
        assert_eq!(vec.len(), 1);

        let header = unsafe { allocator.start.cast::<Header>().as_ref() };
        assert_eq!(
            header.data_length as usize,
            vec.capacity() * core::mem::size_of::<u32>()
        );
        assert_eq!(header.prefix_len, 0);
        assert_eq!(
            header.flags,
            HeaderFlags::OCCUPIED | HeaderFlags::NEXT_INITIALIZED
        );

        drop(vec);

        let header = unsafe { allocator.start.cast::<Header>().as_ref() };
        assert_eq!(header.flags, HeaderFlags::NEXT_INITIALIZED);
    });
}

#[test]
fn test_shrink_no_alloc() {
    extern crate alloc;
    use alloc::vec::Vec;

    with_randomized_memory([0u8; 100], |allocator| {
        let mut vec = Vec::new_in(&allocator);
        assert!(vec.is_empty());
        vec.push(1u32);
        assert_eq!(vec.len(), 1);

        let header = unsafe { allocator.start.cast::<Header>().as_ref() };
        assert_eq!(
            header.data_length as usize,
            vec.capacity() * core::mem::size_of::<u32>()
        );
        assert_eq!(header.prefix_len, 0);
        assert_eq!(
            header.flags,
            HeaderFlags::OCCUPIED | HeaderFlags::NEXT_INITIALIZED
        );

        let allocator_offset = allocator.offset.lock().copy();
        let previous_size = vec.capacity() * core::mem::size_of::<u32>();

        vec.shrink_to_fit();

        // Make sure that the size is actually different
        assert!(previous_size != vec.capacity() * core::mem::size_of::<u32>());

        let header = unsafe { allocator.start.cast::<Header>().as_ref() };
        assert_eq!(
            header.data_length as usize,
            vec.capacity() * core::mem::size_of::<u32>()
        );
        assert_eq!(header.prefix_len, 0);
        assert_eq!(
            header.flags,
            HeaderFlags::OCCUPIED | HeaderFlags::NEXT_INITIALIZED
        );

        // assert that the allocator did not change its offset
        assert_eq!(allocator_offset, allocator.offset.lock().copy());
    });
}

#[test]
fn test_oom() {
    use std::alloc::Allocator as _;

    with_randomized_memory([0u8; 32], |allocator| {
        let err_layout = Layout::from_size_align(20, 1).unwrap();
        let ok_layout = Layout::from_size_align(1, 1).unwrap();

        // Cannot allocate too much
        assert!(allocator.allocate(err_layout).is_err());

        // can allocate `ok_layout`
        let allocated = allocator
            .allocate(ok_layout)
            .expect("Could not allocate ok_layout");
        assert_eq!(allocator.offset.lock().copy(), 0);
        // but only once
        assert!(allocator.allocate(ok_layout).is_err());

        // deallocate the `allocated`
        unsafe {
            allocator.deallocate(allocated.cast(), ok_layout);
        }
        // now we should be able to allocate it again
        assert!(allocator.allocate(ok_layout).is_ok());
    });
}

#[test]
fn test_double_header_initialize() {
    use std::alloc::Allocator as _;

    with_randomized_memory([0u8; 64], |allocator| {
        let ok_layout = Layout::from_size_align(1, 1).unwrap();

        // our buffer fits exactly 2x `ok_layout`
        let first = allocator
            .allocate(ok_layout)
            .expect("Could not allocate first");
        let second = allocator
            .allocate(ok_layout)
            .expect("Could not allocate second");

        // the allocator should be pointing at the start
        assert_eq!(allocator.offset.lock().copy(), 0);

        // a new allocation should fail
        assert!(allocator.allocate(ok_layout).is_err());

        // deallocate the 2 pointers
        unsafe {
            allocator.deallocate(first.cast(), ok_layout);
            allocator.deallocate(second.cast(), ok_layout);
        }

        // allocate the 2 blocks again
        let _first = allocator
            .allocate(ok_layout)
            .expect("Could not allocate first");
        let _second = allocator
            .allocate(ok_layout)
            .expect("Could not allocate second");
    });
}
