#![no_std]
#![feature(strict_provenance)]

use core::{marker::PhantomData, num::NonZeroUsize, ptr::NonNull};
use custom_debug_derive::Debug;

pub struct Atags<'a> {
    addr: NonNull<()>,
    pd: PhantomData<&'a ()>,
}

impl<'a> Atags<'a> {
    pub unsafe fn new(addr: NonNull<()>) -> Self {
        Self {
            addr,
            pd: PhantomData,
        }
    }

    pub fn iter(&'a self) -> AtagIter<'a> {
        AtagIter {
            _atags: self,
            addr: Some(self.addr),
        }
    }
}

pub struct AtagIter<'a> {
    _atags: &'a Atags<'a>,
    addr: Option<NonNull<()>>,
}

impl<'a> Iterator for AtagIter<'a> {
    type Item = Atag<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let addr = self.addr?;
        let header = unsafe { addr.cast::<AtagHeader>().as_ref() };
        let byte_length = (2 + ((header.size + 3) / 4)) as usize;
        let result = {
            let addr = addr.map_addr(|a| unsafe {
                NonZeroUsize::new_unchecked(a.get() + core::mem::size_of::<AtagHeader>())
            });
            match header.tag {
                0x54410001 => {
                    if header.size == 2 {
                        // empty core
                        None
                    } else {
                        Some(Atag::Core(unsafe { addr.cast().as_ref() }))
                    }
                }
                0x54410002 => Some(Atag::Memory(unsafe { addr.cast().as_ref() })),
                0x54410003 => Some(Atag::VideoText(unsafe { addr.cast().as_ref() })),
                0x54410004 => Some(Atag::RamDisk(unsafe { addr.cast().as_ref() })),
                0x54410005 => Some(Atag::InitRd2(unsafe { addr.cast().as_ref() })),
                0x54410006 => Some(Atag::Serial(unsafe { addr.cast().as_ref() })),
                0x54410007 => Some(Atag::Revision(unsafe { addr.cast().as_ref() })),
                0x54410008 => Some(Atag::VideoLfb(unsafe { addr.cast().as_ref() })),
                0x54410009 => Some(Atag::CommandLine(unsafe {
                    core::slice::from_raw_parts(addr.cast().as_ptr(), byte_length)
                })),
                0 => None,
                _ => {
                    let data =
                        unsafe { core::slice::from_raw_parts(addr.cast().as_ptr(), byte_length) };
                    Some(Atag::Unknown { header, data })
                }
            }
        };
        self.addr = if result.is_none() {
            None
        } else {
            Some(addr.map_addr(|a| unsafe {
                NonZeroUsize::new_unchecked(a.get() + (header.size * 4) as usize)
            }))
        };
        result
    }
}

#[derive(Debug)]
pub enum Atag<'a> {
    Core(&'a AtagCore),
    Memory(&'a AtagMemory),
    VideoText(&'a AtagVideoText),
    RamDisk(&'a AtagRamDisk),
    InitRd2(&'a AtagInitRd2),
    Serial(&'a AtagSerial),
    Revision(&'a AtagRevision),
    VideoLfb(&'a AtagVideoLfb),
    CommandLine(&'a [u8]),
    Unknown {
        header: &'a AtagHeader,
        data: &'a [u8],
    },
}

#[repr(C)]
#[derive(Debug)]
pub struct AtagHeader {
    size: u32,
    #[debug(format = "0x{:08X}")]
    tag: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct AtagCore {
    pub flags: u32,
    #[debug(format = "0x{:04X}")]
    pub page_size: u32,
    #[debug(format = "0x{:08X}")]
    pub root_device_number: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct AtagMemory {
    #[debug(format = "0x{:08X}")]
    pub size: u32,
    #[debug(format = "0x{:08X}")]
    pub start: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct AtagVideoText {
    pub width: u8,
    pub height: u8,
    pub video_page: u16,
    pub video_mode: u8,
    pub video_cols: u8,
    pub video_ega_bx: u16,
    pub video_lines: u8,
    pub video_isvga: u8,
    pub video_points: u16,
}

#[repr(C)]
#[derive(Debug)]
pub struct AtagRamDisk {
    pub flags: u32,
    #[debug(format = "{} kb")]
    pub size: u32,
    #[debug(format = "0x{:08X}")]
    pub start: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct AtagInitRd2 {
    #[debug(format = "0x{:08X}")]
    pub start: u32,
    #[debug(format = "0x{:08X}")]
    pub size: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct AtagSerial {
    pub low: u32,
    pub high: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct AtagRevision {
    pub revision: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct AtagVideoLfb {
    pub width: u16,
    pub height: u16,
    pub depth: u16,
    pub line_length: u16,
    pub base: u32,
    pub size: u32,
    pub red_size: u8,
    pub red_pos: u8,
    pub green_size: u8,
    pub green_pos: u8,
    pub blue_size: u8,
    pub blue_pos: u8,
    pub rsvd_size: u8,
    pub rsvd_pos: u8,
}
