#![no_std]
#![feature(strict_provenance)]
#![warn(clippy::pedantic, unsafe_op_in_unsafe_fn)]

mod framebuffer;
mod mailbox;
mod peripherals;

use core::{num::NonZeroUsize, ptr::NonNull};
use framebuffer::PixelOrder;
use mailbox::{items, Channel};
use peripherals::Peripherals;

pub use framebuffer::{Color, FrameBuffer};

pub struct VideoCore<'a> {
    peripherals: &'a mut Peripherals,
}

impl<'a> VideoCore<'a> {
    /// Create a new instance of the `VideoCore`.
    ///
    /// The lifetime that this struct uses is artificial can be tied to e.g. a memory map lifetime.
    ///
    /// **WARNING**: Only once instance of a `VideoCore` should exist.
    ///
    /// # Safety
    ///
    /// `base` must point at a valid MMIO base (e.g. `0x2000_0000` for raspberry pi 1 or `0x3f000_0000` for raspberry pi 2 and up)
    ///
    /// Only once instance of a `VideoCore` should exist.
    #[must_use]
    pub unsafe fn new(base: NonNull<()>) -> Self {
        Self {
            peripherals: unsafe {
                // Safety: the caller must ensure that `base` is a valid MMIO address, which means that `base + 0xB880` is also a valid address
                base.map_addr(|addr| NonZeroUsize::new_unchecked(addr.get() + 0xB880))
                    .cast()
                    // Safety: `base + 0xB880` should be a valid VCMAILBOX address
                    .as_mut()
            },
        }
    }

    pub fn allocate_framebuffer(&mut self, width: u32, height: u32, depth: u32) -> FrameBuffer {
        let response = mailbox::Request::new()
            .set(items::FramebufferScreenSize { width, height })
            .set(items::FramebufferDepth { depth })
            .set(items::FramebufferPixelOrder {
                order: PixelOrder::RGB,
            })
            .get(items::FramebufferAllocate::alignment(16))
            .send(self.peripherals, Channel::PropertyToVC);

        let (screen_size, response) = response.pop();
        let (depth, response) = response.pop();
        let (pixel_order, response) = response.pop();
        let allocation = response.pop();

        FrameBuffer::new(
            allocation.addr,
            allocation.size,
            screen_size.width,
            screen_size.height,
            depth.depth,
            pixel_order.order,
        )
    }
}
