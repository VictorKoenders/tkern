use core::{alloc::Layout, cell::RefCell, ops::Range};

pub struct BumpAllocator {
    inner: RefCell<Inner>,
}

struct Inner {
    frames: [Range<usize>; 10],
    last_frame_index: usize,
    last_frame_offset: usize,
}

impl BumpAllocator {
    pub const fn uninit() -> Self {
        Self {
            inner: RefCell::new(Inner {
                frames: [0..0, 0..0, 0..0, 0..0, 0..0, 0..0, 0..0, 0..0, 0..0, 0..0],
                last_frame_index: 0,
                last_frame_offset: 0,
            }),
        }
    }

    pub unsafe fn init(&mut self, boot_info: &multiboot2::BootInformation) {
        let elf_sections_tag = boot_info
            .elf_sections_tag()
            .expect("Elf-sections tag required");
        let kernel_start = elf_sections_tag
            .sections()
            .map(|s| s.start_address())
            .min()
            .unwrap() as usize;
        let kernel_end = elf_sections_tag
            .sections()
            .map(|s| s.end_address())
            .max()
            .unwrap() as usize;

        let multiboot_start = boot_info.start_address();
        let multiboot_end = boot_info.end_address();

        // Don't allocate memory in the region of multiboot, kernel etc
        let disallowed_ranges = &[multiboot_start..multiboot_end, kernel_start..kernel_end];

        vga_println!("Allocating up to 10 frames");

        let mut frame_index = 0;
        let areas = boot_info.memory_map_tag().unwrap().memory_areas();
        let mut inner = self.inner.borrow_mut();
        for area in areas {
            let range = area.start_address() as usize..area.end_address() as usize;
            if let Some(valid_range) = exclude_ranges(range, disallowed_ranges) {
                // we have a valid memory range we can use

                inner.frames[frame_index] = valid_range.clone();
                frame_index += 1;
                let (size, magnitude) = printable_size(valid_range.end - valid_range.start);
                vga_println!(
                    "{}) 0x{:x}..0x{:x} ({} {})",
                    frame_index,
                    valid_range.start,
                    valid_range.end,
                    size,
                    magnitude
                );

                if frame_index == inner.frames.len() {
                    // hit max frames
                    break;
                }
            }
        }

        if inner.frames[0].start == 0x00 {
            // don't start at 0x0, this is reserved as a nullpointer
            inner.last_frame_offset = 0x01;
        }
    }

    pub unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut inner = self.inner.borrow_mut();
        loop {
            if inner.last_frame_index >= inner.frames.len() {
                // we've exhausted all frames, return a null pointer
                return core::ptr::null_mut();
            }
            let frame = &inner.frames[inner.last_frame_index];

            let mut desired_start = frame.start + inner.last_frame_offset;

            // make sure desired_start is properly aligned
            let offset_to_last_valid_align = desired_start % layout.align();
            if offset_to_last_valid_align != 0 {
                // we're not aligned, update the desired_start to the next valid align
                let remaining_align = layout.align() - offset_to_last_valid_align;
                desired_start += remaining_align;
                debug_assert_eq!(desired_start % layout.align(), 0);
            }

            let desired_range = desired_start..desired_start + layout.size();

            if desired_range.end <= frame.end {
                // ok, bump the last_frame_index
                inner.last_frame_offset = desired_range.end + 1;
                return desired_range.start as *mut u8;
            }
            // doesn't fit in the current frame, try bumping the frame and try again
            inner.last_frame_index += 1;
        }
    }
    pub unsafe fn dealloc(&self, _ptr: *const u8, _layout: Layout) {
        // No deallocation yet
    }
}

fn exclude_ranges(
    start_range: Range<usize>,
    excluded_ranges: &[Range<usize>],
) -> Option<Range<usize>> {
    let mut result = start_range;
    for range in excluded_ranges {
        if range.end < result.start || range.start > result.end {
            // no overlap
            continue;
        } else if range.end >= result.end {
            if range.start > result.start {
                // range ends past our result, slice off the end of our result
                result = result.start..range.start;
            } else {
                // range completely overlaps us, we can't use this result
                return None;
            }
        } else if range.start <= result.start {
            if range.end < result.end {
                // range starts earlier than our result, slice off the start of our result
                result = range.end..result.end;
            } else {
                // range completely overlaps us, we can't use this result
                return None;
            }
        } else {
            // the range is inside our result
            // too complicated for now, we'll just return None
            return None;
        }
    }
    Some(result)
}

fn printable_size(size: usize) -> (usize, &'static str) {
    if size < 10 * 1024 {
        // less than 10KB, print as bytes
        (size, "B")
    } else if size < 10 * 1024 * 1024 {
        // less than 10MB, print as KB
        (size / 1024, "KB")
    } else if size < 10 * 1024 * 1024 * 1024 {
        // less than 10GB, print as MB
        (size / 1024 / 1024, "MB")
    } else if size < 10 * 1024 * 1024 * 1024 * 1024 {
        // less than 10TB, print as GB
        (size / 1024 / 1024 / 1024, "GB")
    } else {
        // TODO: support TB
        (size, "Way too big")
    }
}
