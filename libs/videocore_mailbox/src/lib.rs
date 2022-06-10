#![no_std]
#![feature(strict_provenance)]
#![warn(clippy::pedantic, unsafe_op_in_unsafe_fn)]

mod framebuffer;
mod mailbox;
mod peripherals;

use core::{num::NonZeroUsize, ptr::NonNull};
use mailbox::Channel;
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
            .set_framebuffer_screen_size(width, height)
            .set_framebuffer_virtual_screen_size(width, height)
            .set_framebuffer_depth(depth)
            .set_framebuffer_depth(depth)
            .set_framebuffer_pixel_order(framebuffer::PixelOrder::RGB)
            .allocate_framebuffer(16)
            .send(self.peripherals, Channel::PropertyToVC);
        let mut width = width;
        let mut height = height;
        let mut depth = depth;
        let mut pixel_order = framebuffer::PixelOrder::RGB;
        let mut framebuffer_pointer = 0;
        let mut framebuffer_size = 0;
        for item in response.iter() {
            match item.kind() {
                mailbox::ItemKind::SetFramebufferScreenSize => {
                    width = item.u32(0);
                    height = item.u32(1);
                }
                mailbox::ItemKind::SetFramebufferDepth => {
                    depth = item.u32(0);
                }
                mailbox::ItemKind::SetFramebufferPixelOrder => {
                    pixel_order = if item.u32(0) == 0x01 {
                        framebuffer::PixelOrder::RGB
                    } else {
                        framebuffer::PixelOrder::BGR
                    };
                }
                mailbox::ItemKind::AllocateFramebuffer => {
                    framebuffer_pointer = item.u32(0);
                    framebuffer_size = item.u32(1);
                }
                mailbox::ItemKind::SetFramebufferVirtualScreenSize => {}
                _ => unreachable!("Unknown item kind {:?}", item.kind()),
            }
        }
        FrameBuffer::new(
            framebuffer_pointer,
            framebuffer_size,
            width,
            height,
            depth,
            pixel_order,
        )
    }
}
