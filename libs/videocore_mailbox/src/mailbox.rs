#![allow(clippy::module_name_repetitions)]

use crate::peripherals::{Peripherals, Write};
use aligned::{Aligned, A16};
use core::marker::PhantomData;

pub mod items;

pub trait MailboxItem {
    const HARDWARE_DEVICE: u32;
    const COMMAND: u32;
    const SIZE: u32;
    fn write(&self, slice: &mut [u8]);
    fn get(slice: &[u8]) -> Self;
}
pub trait Gettable {}
pub trait Settable {}
pub trait Testable {}

pub trait MailboxItemTuple {
    const SIZE: u32;
    fn write(&self, slice: &mut [u8]);
}

pub struct Set<T: Settable>(T);
pub struct Test<T: Testable>(T);
pub struct Get<T: Gettable>(T);

pub trait ValidProperty {
    type Inner: MailboxItem;

    const TAG: u32;
    const SIZE: u32 = 12 + <Self::Inner as MailboxItem>::SIZE;

    fn id() -> u32 {
        let hardware = <Self::Inner as MailboxItem>::HARDWARE_DEVICE;
        let command = <Self::Inner as MailboxItem>::COMMAND;
        ((hardware << 16) & 0x000F_0000)
            | ((Self::TAG << 12) & 0x0000_F000)
            | (command & 0x0000_0FFF)
    }
    fn inner(&self) -> &Self::Inner;
    fn write(&self, slice: &mut [u8]) {
        let mailbox_len = <Self::Inner as MailboxItem>::SIZE;
        debug_assert_eq!(slice.len(), 12 + mailbox_len as usize);
        slice[..4].copy_from_slice(&Self::id().to_le_bytes());
        slice[4..8].copy_from_slice(&mailbox_len.to_le_bytes());
        slice[8..12].copy_from_slice(&0u32.to_le_bytes()); // request is always zero
        self.inner().write(&mut slice[12..]);
    }
}

impl<T: Gettable + MailboxItem> ValidProperty for Get<T> {
    type Inner = T;

    const TAG: u32 = 0;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }
}
impl<T: Testable + MailboxItem> ValidProperty for Test<T> {
    type Inner = T;

    const TAG: u32 = 4;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }
}
impl<T: Settable + MailboxItem> ValidProperty for Set<T> {
    type Inner = T;

    const TAG: u32 = 8;

    fn inner(&self) -> &Self::Inner {
        &self.0
    }
}

pub(super) struct Response<T> {
    bytes: [u8; 144],
    ptr: usize,
    pd: PhantomData<T>,
}

#[derive(Debug)]
pub(super) struct Request<T> {
    data: T,
}

#[allow(dead_code, clippy::unused_self)]
impl Request<()> {
    pub fn new() -> Self {
        Self { data: () }
    }
    pub fn set<Append>(self, append: Append) -> Request<(Set<Append>,)>
    where
        Append: MailboxItem + Settable,
    {
        Request {
            data: (Set(append),),
        }
    }
    pub fn get<Append>(self, append: Append) -> Request<(Get<Append>,)>
    where
        Append: MailboxItem + Gettable,
    {
        Request {
            data: (Get(append),),
        }
    }
    pub fn test<Append>(self, append: Append) -> Request<(Test<Append>,)>
    where
        Append: MailboxItem + Testable,
    {
        Request {
            data: (Test(append),),
        }
    }
}

macro_rules! impl_request_tuple {
    ($($tup:ident), *) => {
        #[allow(dead_code, non_snake_case)]
        impl<$($tup,)*> Request<($($tup,) *)> {
            pub fn set<Append>(self, append: Append) -> Request<($($tup,) *Set<Append>, )>
                where Append: MailboxItem + Settable {
                let ($($tup,) *) = self.data;
                Request {
                    data: ($($tup,) * Set(append), ),
                }
            }
            pub fn get<Append>(self, append: Append) -> Request<($($tup,)* Get<Append>, )>
                where Append: MailboxItem + Gettable {
                let ($($tup,) *) = self.data;
                Request {
                    data: ($($tup,)* Get(append), ),
                }
            }
            pub fn test<Append>(self, append: Append) -> Request<($($tup,)* Test<Append>, )>
                where Append: MailboxItem + Testable {
                let ($($tup,) *) = self.data;
                Request {
                    data: ($($tup,)* Test(append), ),
                }
            }
        }

    }
}

impl_request_tuple!(A);
impl_request_tuple!(A, B);
impl_request_tuple!(A, B, C);
impl_request_tuple!(A, B, C, D);
impl_request_tuple!(A, B, C, D, E);

macro_rules! impl_mailbox_tuple {
    ($($tup:ident), *) => {
        #[allow(dead_code, non_snake_case, unused_assignments)]
        impl<$($tup, )*> MailboxItemTuple for ($($tup, )*) where $($tup: ValidProperty, )* {
            const SIZE: u32 = 0 $(+ $tup::SIZE)*;
            fn write(&self, slice: &mut [u8]) {
                debug_assert_eq!(slice.len(), Self::SIZE as usize);
                let ($($tup, )*) = self;
                let mut start = 0;
                $({
                    let size = $tup::SIZE as usize;
                    let slice = &mut slice[start .. start + size];
                    start += size;
                    $tup.write(slice);
                })*
            }
        }
    }
}
impl_mailbox_tuple!(A);
impl_mailbox_tuple!(A, B);
impl_mailbox_tuple!(A, B, C);
impl_mailbox_tuple!(A, B, C, D);
impl_mailbox_tuple!(A, B, C, D, E);
impl_mailbox_tuple!(A, B, C, D, E, F);

fn get_property_from_slice<T: ValidProperty>(slice: &[u8], ptr: usize) -> (T::Inner, usize) {
    if cfg!(debug_assertions) {
        let mut tag = [0u8; 4];
        tag.copy_from_slice(&slice[ptr..ptr + 4]);
        let tag = u32::from_le_bytes(tag);
        assert_eq!(
            tag,
            T::id(),
            "{} should be ID 0x{:08X} but is 0x{:08X} (offset {})",
            core::any::type_name::<T>(),
            T::id(),
            tag,
            ptr
        );
    }
    let len = <T::Inner as MailboxItem>::SIZE;
    let start = ptr + 12;
    let end = start + len as usize;
    let slice = &slice[start..end];
    (<T::Inner as MailboxItem>::get(slice), end)
}

macro_rules! impl_response_pop {
    ($front:ident $(, $rem:ident)*) => {
        #[allow(dead_code)]
        impl<$front, $($rem, )*> Response<($front, $($rem, )*)>
            where $front: ValidProperty {
            pub fn pop(self) -> (<$front as ValidProperty>::Inner, Response<($($rem, )*)>) {
                let Self { bytes, ptr, .. } = self;
                let (result, ptr) = get_property_from_slice::<$front>(&bytes, ptr);
                (
                    result,
                    Response {
                        bytes,
                        pd: PhantomData,
                        ptr
                    }
                )
            }
        }
    }
}
impl_response_pop!(A, B);
impl_response_pop!(A, B, C);
impl_response_pop!(A, B, C, D);
impl_response_pop!(A, B, C, D, E);
impl_response_pop!(A, B, C, D, E, F);

impl<RET> Response<(RET,)>
where
    RET: ValidProperty,
{
    pub fn pop(self) -> <RET as ValidProperty>::Inner {
        let Self { bytes, ptr, .. } = self;
        let (result, ptr) = get_property_from_slice::<RET>(&bytes, ptr);
        debug_assert_eq!(&bytes[ptr..ptr + 4], &[0u8; 4]);
        result
    }
}
impl<T> Response<T> {
    pub(crate) fn from_bytes(bytes: [u8; 144]) -> Self {
        Self {
            bytes,
            pd: PhantomData,
            ptr: 8,
        }
    }
}

impl<T: MailboxItemTuple> Request<T> {
    fn into_bytes(self) -> Aligned<A16, [u8; 144]> {
        let length = 8 + T::SIZE;
        let padding = 1 + if (length + 1) % 16 == 0 {
            0
        } else {
            16 - ((length + 1) % 16)
        };

        debug_assert!(length <= 144);
        let mut buffer: Aligned<A16, [u8; 144]> = Aligned([0u8; 144]);

        buffer[0..4].copy_from_slice(&((length + padding) as u32).to_le_bytes());
        buffer[4..8].copy_from_slice(&0u32.to_le_bytes());
        self.data.write(&mut buffer[8..length as usize]);
        buffer
    }
    pub fn send(self, peripherals: &mut Peripherals, channel: Channel) -> Response<T> {
        let buffer = self.into_bytes();

        let write = Write::new(
            u32::try_from(core::ptr::addr_of!(buffer) as usize).unwrap(),
            channel as u8,
        );
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
                let bytes = if data != 0 {
                    unsafe { core::ptr::read_volatile(data as usize as *const _) }
                } else if data as usize == core::ptr::addr_of!(buffer) as usize {
                    unsafe { core::ptr::read_volatile(&*buffer) }
                } else {
                    continue;
                };
                return Response::from_bytes(bytes);
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

#[test]
fn test_request_builder_with_known_values() {
    let request = Request::new()
        .set(items::FramebufferScreenSize {
            width: 640,
            height: 480,
        })
        .set(items::FramebufferVirtualScreenSize {
            width: 640,
            height: 480,
        })
        .set(items::FramebufferDepth { depth: 24 });
    let bytes = request.into_bytes();
    #[allow(clippy::explicit_auto_deref)] // clippy is wrong
    let u32_bytes = bytemuck::cast_slice::<u8, u32>(&*bytes);
    #[rustfmt::skip]
    assert_eq!(
        &u32_bytes[..20],
        &[
            80, // The whole buffer is 80 bytes
            0,  // This is a request, so the request/response code is 0
            0x0004_8003, 8, 0, 640, 480, // This tag sets the screen size to 640x480
            0x0004_8004, 8, 0, 640, 480, // This tag sets the virtual screen size to 640x480
            0x0004_8005, 4, 0, 24, // This tag sets the depth to 24 bits
            0,  // This is the end tag
            0, 0, 0 // This pads the message to by 16 byte aligned
        ]
    );

    let request = Request::new()
        .set(items::FramebufferScreenSize {
            width: 800,
            height: 600,
        })
        .set(items::FramebufferVirtualScreenSize {
            width: 800,
            height: 600,
        })
        .set(items::FramebufferDepth { depth: 24 })
        .set(items::FramebufferDepth { depth: 24 })
        .set(items::FramebufferPixelOrder {
            order: crate::framebuffer::PixelOrder::RGB,
        })
        .get(items::FramebufferAllocate::alignment(16));
    let bytes = request.into_bytes();
    assert_eq!(
        &bytes[..128],
        &[
            128, 0, 0, 0, 0, 0, 0, 0, 3, 128, 4, 0, 8, 0, 0, 0, 0, 0, 0, 0, 32, 3, 0, 0, 88, 2, 0,
            0, 4, 128, 4, 0, 8, 0, 0, 0, 0, 0, 0, 0, 32, 3, 0, 0, 88, 2, 0, 0, 5, 128, 4, 0, 4, 0,
            0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 5, 128, 4, 0, 4, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 6,
            128, 4, 0, 4, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 4, 0, 8, 0, 0, 0, 0, 0, 0, 0, 16,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]
    );

    #[allow(clippy::type_complexity)] // OK for testing
    let response: Response<(
        Set<items::FramebufferScreenSize>,
        Set<items::FramebufferVirtualScreenSize>,
        Set<items::FramebufferDepth>,
        Set<items::FramebufferDepth>,
        Set<items::FramebufferPixelOrder>,
        Get<items::FramebufferAllocate>,
    )> = Response::from_bytes(*bytes);

    let (screen_size, response) = response.pop();
    assert_eq!(
        screen_size,
        items::FramebufferScreenSize {
            width: 800,
            height: 600
        }
    );
    let (virtual_screen_size, response) = response.pop();
    assert_eq!(
        virtual_screen_size,
        items::FramebufferVirtualScreenSize {
            width: 800,
            height: 600
        }
    );
    let (depth, response) = response.pop();
    assert_eq!(depth, items::FramebufferDepth { depth: 24 });
    let (depth, response) = response.pop();
    assert_eq!(depth, items::FramebufferDepth { depth: 24 });
    let (pixel_order, response) = response.pop();
    assert_eq!(
        pixel_order,
        items::FramebufferPixelOrder {
            order: crate::framebuffer::PixelOrder::RGB
        }
    );
    let allocate = response.pop();
    assert_eq!(allocate, items::FramebufferAllocate::alignment(16));
}
