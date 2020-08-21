use super::{Frame, FrameAllocator, PhysicalAddress};
use multiboot2::{MemoryArea, MemoryAreaIter};

pub struct AreaFrameAllocator<'a> {
    next_free_frame: Frame,
    current_area: Option<&'a MemoryArea>,
    areas: MemoryAreaIter<'a>,
    kernel_start: Frame,
    kernel_end: Frame,
    multiboot_start: Frame,
    multiboot_end: Frame,
}

impl<'a> AreaFrameAllocator<'a> {
    pub fn new(boot_info: &'a multiboot2::BootInformation) -> AreaFrameAllocator {
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

        let mut allocator = AreaFrameAllocator {
            next_free_frame: Frame::containing_address(PhysicalAddress::new(0)),
            current_area: None,
            areas: boot_info.memory_map_tag().unwrap().memory_areas(),
            kernel_start: Frame::containing_address(PhysicalAddress::new(kernel_start)),
            kernel_end: Frame::containing_address(PhysicalAddress::new(kernel_end)),
            multiboot_start: Frame::containing_address(PhysicalAddress::new(multiboot_start)),
            multiboot_end: Frame::containing_address(PhysicalAddress::new(multiboot_end)),
        };
        allocator.choose_next_area();
        allocator
    }
    fn choose_next_area(&mut self) {
        self.current_area = self
            .areas
            .clone()
            .filter(|area| {
                let address = area.end_address() - 1;
                Frame::containing_address(PhysicalAddress::new(address as usize))
                    >= self.next_free_frame
            })
            .min_by_key(|area| area.start_address());

        if let Some(area) = self.current_area {
            let start_frame =
                Frame::containing_address(PhysicalAddress::new(area.start_address() as usize));
            if self.next_free_frame < start_frame {
                self.next_free_frame = start_frame;
            }
        }
    }
}

impl<'a> FrameAllocator for AreaFrameAllocator<'a> {
    fn allocate_frame(&mut self) -> Option<Frame> {
        if let Some(area) = self.current_area {
            // "Clone" the frame to return it if it's free. Frame doesn't
            // implement Clone, but we can construct an identical frame.
            let frame = Frame {
                number: self.next_free_frame.number,
            };

            // the last frame of the current area
            let current_area_last_frame = {
                let address = area.end_address() - 1;
                Frame::containing_address(PhysicalAddress::new(address as usize))
            };

            if frame > current_area_last_frame {
                // all frames of current area are used, switch to next area
                self.choose_next_area();
            } else if frame >= self.kernel_start && frame <= self.kernel_end {
                // `frame` is used by the kernel
                self.next_free_frame = Frame {
                    number: self.kernel_end.number + 1,
                };
            } else if frame >= self.multiboot_start && frame <= self.multiboot_end {
                // `frame` is used by the multiboot information structure
                self.next_free_frame = Frame {
                    number: self.multiboot_end.number + 1,
                };
            } else {
                // frame is unused, increment `next_free_frame` and return it
                self.next_free_frame.number += 1;
                return Some(frame);
            }
            // `frame` was not valid, try it again with the updated `next_free_frame`
            self.allocate_frame()
        } else {
            None // no free frames left
        }
    }

    fn deallocate_frame(&mut self, _frame: Frame) {
        unimplemented!()
    }
}
