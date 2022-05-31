use crate::framebuffer::PixelOrder;
use crate::peripherals::{Peripherals, Write};
use aligned::{Aligned, A16};
use tkern_derive_utils::RegEnum;

pub(super) struct Response {
    pub bytes: [u8; 144],
}

impl Response {
    pub fn iter(&self) -> ResponseIter {
        ResponseIter {
            bytes: &self.bytes[8..],
        }
    }
}

pub(super) struct ResponseIter<'a> {
    bytes: &'a [u8],
}

impl<'a> Iterator for ResponseIter<'a> {
    type Item = Item<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let identifier = *bytemuck::try_from_bytes::<u32>(self.bytes.get(0..4)?).ok()?;
        if identifier == 0 {
            return None;
        }
        let length = *bytemuck::try_from_bytes::<u32>(&self.bytes[4..8]).ok()?;
        let (slice, remaining) = self.bytes.split_at((12 + length) as usize);
        self.bytes = remaining;
        Some(Item(slice))
    }
}

#[derive(RegEnum, Debug)]
#[non_exhaustive]
pub(super) enum ItemKind {
    /// 0x00000001: Firmware revision
    FirmwareRevision,
    /// 0x00010001: Board model
    BoardModel,
    /// 0x00010002: Board revision
    BoardRevision,
    /// 0x00010003: Board MAC address
    BoardMacAddress,
    /// 0x00010004: Board Serial
    BoardSerial,
    /// 0x00010005: Get ARM Memory
    ArmMemory,
    /// 0x00010006: Get video core Memory
    VideoCoreMemory,

    /// 0x00040001: Allocate framebuffer
    AllocateFramebuffer,
    /// 0x00048003: Set framebuffer screen size
    SetFramebufferScreenSize,
    /// 0x00048004: Set framebuffer virtual screen size
    SetFramebufferVirtualScreenSize,
    /// 0x00048005: Set framebuffer depth
    SetFramebufferDepth,
    /// 0x00048006: Set framebuffer pixel order
    SetFramebufferPixelOrder,

    /// 0x00030041: Get LED status
    LedStatus,

    Unknown(u32),
}

impl ItemKind {
    pub fn to_bytes(self) -> [u8; 4] {
        (self.to_u64() as u32).to_le_bytes()
    }
}

pub(super) struct Item<'a>(&'a [u8]);
impl<'a> Item<'a> {
    pub fn kind(&self) -> ItemKind {
        let identifier = *bytemuck::from_bytes::<u32>(&self.0[0..4]);
        ItemKind::new(identifier as u64)
    }
    pub fn bytes(&self) -> &[u8] {
        &self.0[12..]
    }
    pub fn u32s(&self) -> &[u32] {
        bytemuck::cast_slice(self.bytes())
    }
    pub fn u32(&self, index: usize) -> u32 {
        self.u32s()[index]
    }
}

impl<'a> core::fmt::Display for Item<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::write!(f, "{:?}: {:02X?}", self.kind(), self.bytes())
    }
}

#[derive(Debug)]
pub(super) struct Request {
    pub bytes: [u8; 144],
    index: usize,
}

impl Request {
    pub fn new() -> Self {
        Self {
            bytes: [0u8; 144],
            index: 8,
        }
    }

    fn add_slice(&mut self, slice: &[u8]) {
        self.bytes[self.index..self.index + slice.len()].copy_from_slice(slice);
        self.index += slice.len();
    }

    fn add_tag<const N: usize>(mut self, tag: ItemKind, buffer: [u8; N]) -> Self {
        self.add_slice(&tag.to_bytes()); // tag
        self.add_slice(&(N as u32).to_le_bytes()); // request/response byte size
        self.add_slice(&[0u8; 4]); // 0 = request
        self.add_slice(&buffer);
        self
    }

    // pub fn get_firmware_revision(self) -> Self {
    //     self.add_tag(
    //         ItemKind::FirmwareRevision,
    //         [0u8; 4]
    //     )
    // }
    // pub fn get_board_model(self) -> Self {
    //     self.add_tag(
    //         ItemKind::BoardModel,
    //         [0u8; 4]
    //     )
    // }
    // pub fn get_board_revision(self) -> Self {
    //     self.add_tag(
    //         ItemKind::BoardRevision,
    //         [0u8; 4]
    //     )
    // }
    // pub fn get_mac_address(self) -> Self {
    //     self.add_tag(
    //         ItemKind::BoardMacAddress,
    //         [0u8; 8]
    //     )
    // }
    // pub fn get_board_serial(self) -> Self {
    //     self.add_tag(
    //         ItemKind::BoardSerial,
    //         [0u8; 8]
    //     )
    // }
    // pub fn get_arm_memory(self) -> Self {
    //     self.add_tag(
    //         ItemKind::ArmMemory,
    //         [0u8; 8]
    //     )
    // }

    // pub fn get_video_core_memory(self) -> Self {
    //     self.add_tag(
    //         ItemKind::VideoCoreMemory,
    //         [0u8; 8]
    //     )
    // }

    pub fn set_framebuffer_screen_size(self, width: u32, height: u32) -> Self {
        let mut buffer = [0u8; 8];
        buffer[..4].copy_from_slice(&width.to_le_bytes());
        buffer[4..].copy_from_slice(&height.to_le_bytes());
        self.add_tag(ItemKind::SetFramebufferScreenSize, buffer)
    }

    pub fn set_framebuffer_virtual_screen_size(self, width: u32, height: u32) -> Self {
        let mut buffer = [0u8; 8];
        buffer[..4].copy_from_slice(&width.to_le_bytes());
        buffer[4..].copy_from_slice(&height.to_le_bytes());
        self.add_tag(ItemKind::SetFramebufferVirtualScreenSize, buffer)
    }

    pub fn set_framebuffer_depth(self, depth: u32) -> Self {
        self.add_tag(ItemKind::SetFramebufferDepth, depth.to_le_bytes())
    }

    pub fn set_framebuffer_pixel_order(self, order: PixelOrder) -> Self {
        self.add_tag(
            ItemKind::SetFramebufferPixelOrder,
            (order as u32).to_le_bytes(),
        )
    }

    pub fn allocate_framebuffer(self, byte_alignment: u32) -> Self {
        let mut buffer = [0u8; 8];
        buffer[..4].copy_from_slice(&byte_alignment.to_le_bytes());
        self.add_tag(ItemKind::AllocateFramebuffer, buffer)
    }

    pub fn send(mut self, peripherals: &mut Peripherals, channel: Channel) -> Response {
        let mut length = self.index as u32;
        if length % 4 != 0 {
            length += 4 - (length % 4);
        }
        assert!(length <= self.bytes.len() as u32);
        self.bytes[0..4].copy_from_slice(&(length as u32).to_le_bytes());
        let buffer = Aligned::<A16, _>(self.bytes);
        let write = Write::new(core::ptr::addr_of!(buffer) as usize as u32, channel as u8);
        while peripherals.status.read().full() {
            cortex_a::asm::nop();
        }
        peripherals.write.write(write);
        loop {
            while peripherals.status.read().empty() {
                cortex_a::asm::nop();
            }
            let read = peripherals.read.read();
            if read.channel() == channel as u8 {
                let data = read.data();
                if data != 0 {
                    return Response {
                        bytes: unsafe { core::ptr::read_volatile(data as usize as *const _) },
                    };
                } else if data == core::ptr::addr_of!(buffer) as usize as u32 {
                    return Response {
                        bytes: unsafe { core::ptr::read_volatile(&*buffer) },
                    };
                }
            }
        }
    }
}

#[non_exhaustive]
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub(super) enum Channel {
    // Power = 0,
    // Framebuffer = 1,
    // VirtualUart = 2,
    // VCHIQ = 3,
    // LEDs = 4,
    // Buttons = 5,
    // TouchScreen = 6,
    PropertyToVC = 8,
    // PropertyFromVC = 9,
}
