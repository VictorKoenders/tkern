use super::{Gettable, MailboxItem, Settable};
use crate::framebuffer::PixelOrder;

#[allow(dead_code)]
mod device {
    pub const FIRMWARE: u32 = 0;
    pub mod firmware {
        pub const REVISION: u32 = 1;
    }
    pub const BOARD: u32 = 1;
    pub mod board {
        pub const MODEL: u32 = 1;
        pub const REVISION: u32 = 2;
        pub const MAC_ADDRESS: u32 = 3;
        pub const SERIAL: u32 = 4;
        pub const ARM_MEMORY: u32 = 5;
        pub const VIDEO_CORE_MEMORY: u32 = 6;
    }

    pub const FRAMEBUFFER: u32 = 4;
    pub mod framebuffer {
        pub const ALLOCATE: u32 = 1;
        pub const SCREEN_SIZE: u32 = 3;
        pub const VIRTUAL_SCREEN_SIZE: u32 = 4;
        pub const DEPTH: u32 = 5;
        pub const PIXEL_ORDER: u32 = 6;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FramebufferScreenSize {
    pub width: u32,
    pub height: u32,
}
impl MailboxItem for FramebufferScreenSize {
    const HARDWARE_DEVICE: u32 = device::FRAMEBUFFER;
    const COMMAND: u32 = device::framebuffer::SCREEN_SIZE;
    const SIZE: u32 = 8;

    fn write(&self, slice: &mut [u8]) {
        debug_assert_eq!(slice.len(), Self::SIZE as usize);
        slice[..4].copy_from_slice(&self.width.to_le_bytes());
        slice[4..].copy_from_slice(&self.height.to_le_bytes());
    }

    fn get(slice: &[u8]) -> Self {
        debug_assert_eq!(slice.len(), Self::SIZE as usize);
        Self {
            width: get_u32(&slice[..4]),
            height: get_u32(&slice[4..]),
        }
    }
}
impl Settable for FramebufferScreenSize {}
impl Gettable for FramebufferScreenSize {}

#[derive(Debug, PartialEq, Eq)]
pub struct FramebufferVirtualScreenSize {
    pub width: u32,
    pub height: u32,
}
impl MailboxItem for FramebufferVirtualScreenSize {
    const HARDWARE_DEVICE: u32 = device::FRAMEBUFFER;
    const COMMAND: u32 = device::framebuffer::VIRTUAL_SCREEN_SIZE;
    const SIZE: u32 = 8;

    fn write(&self, slice: &mut [u8]) {
        debug_assert_eq!(slice.len(), Self::SIZE as usize);
        slice[..4].copy_from_slice(&self.width.to_le_bytes());
        slice[4..].copy_from_slice(&self.height.to_le_bytes());
    }

    fn get(slice: &[u8]) -> Self {
        debug_assert_eq!(slice.len(), Self::SIZE as usize);
        Self {
            width: get_u32(&slice[..4]),
            height: get_u32(&slice[4..]),
        }
    }
}
impl Settable for FramebufferVirtualScreenSize {}
impl Gettable for FramebufferVirtualScreenSize {}

#[derive(Debug, PartialEq, Eq)]
pub struct FramebufferDepth {
    pub depth: u32,
}
impl MailboxItem for FramebufferDepth {
    const HARDWARE_DEVICE: u32 = device::FRAMEBUFFER;
    const COMMAND: u32 = device::framebuffer::DEPTH;
    const SIZE: u32 = 4;

    fn write(&self, slice: &mut [u8]) {
        debug_assert_eq!(slice.len(), Self::SIZE as usize);
        slice.copy_from_slice(&self.depth.to_le_bytes());
    }

    fn get(slice: &[u8]) -> Self {
        debug_assert_eq!(slice.len(), Self::SIZE as usize);
        Self {
            depth: get_u32(slice),
        }
    }
}
impl Settable for FramebufferDepth {}
impl Gettable for FramebufferDepth {}

#[derive(Debug, PartialEq, Eq)]
pub struct FramebufferPixelOrder {
    pub order: PixelOrder,
}
impl MailboxItem for FramebufferPixelOrder {
    const HARDWARE_DEVICE: u32 = device::FRAMEBUFFER;
    const COMMAND: u32 = device::framebuffer::PIXEL_ORDER;
    const SIZE: u32 = 4;

    fn write(&self, slice: &mut [u8]) {
        debug_assert_eq!(slice.len(), Self::SIZE as usize);
        let order = self.order as u32;
        slice.copy_from_slice(&order.to_le_bytes());
    }

    fn get(slice: &[u8]) -> Self {
        debug_assert_eq!(slice.len(), Self::SIZE as usize);
        Self {
            order: if get_u32(slice) == PixelOrder::BGR as u32 {
                PixelOrder::BGR
            } else {
                PixelOrder::RGB
            },
        }
    }
}
impl Settable for FramebufferPixelOrder {}
impl Gettable for FramebufferPixelOrder {}

#[derive(Debug, PartialEq, Eq)]
pub struct FramebufferAllocate {
    pub addr: u32,
    pub size: u32,
}
impl FramebufferAllocate {
    pub fn alignment(alignment: u32) -> Self {
        Self {
            addr: alignment,
            size: 0,
        }
    }
}
impl MailboxItem for FramebufferAllocate {
    const HARDWARE_DEVICE: u32 = device::FRAMEBUFFER;
    const COMMAND: u32 = device::framebuffer::ALLOCATE;
    const SIZE: u32 = 8;

    fn write(&self, slice: &mut [u8]) {
        debug_assert_eq!(slice.len(), Self::SIZE as usize);
        slice[..4].copy_from_slice(&self.addr.to_le_bytes());
        slice[4..].copy_from_slice(&self.size.to_le_bytes());
    }

    fn get(slice: &[u8]) -> Self {
        debug_assert_eq!(slice.len(), Self::SIZE as usize);
        Self {
            addr: get_u32(&slice[..4]),
            size: get_u32(&slice[4..]),
        }
    }
}
impl Gettable for FramebufferAllocate {}

fn get_u32(slice: &[u8]) -> u32 {
    assert_eq!(slice.len(), 4);
    let mut arr = [0u8; 4];
    arr.copy_from_slice(slice);
    u32::from_le_bytes(arr)
}
