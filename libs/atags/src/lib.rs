#![doc = include_str!("../README.md")]
#![no_std]
#![cfg_attr(feature = "nightly", feature(strict_provenance))]
#![warn(
    unsafe_op_in_unsafe_fn,
    clippy::pedantic,
    rust_2018_idioms,
    missing_docs
)]

use core::{marker::PhantomData, ptr::NonNull};
use custom_debug_derive::Debug;

/// Handler that points to a memory location that holds the tag definitions.
pub struct Atags<'a> {
    addr: NonNull<()>,
    pd: PhantomData<&'a ()>,
}

impl<'a> Atags<'a> {
    /// Create a new `Atag` struct that points at the given location.
    ///
    /// returned lifetime will be chosen arbitrarily. you can use this to tie this struct to your memory allocator, if you're using a custom one.
    ///
    /// # Safety
    ///
    /// The given address must be a valid atag header.
    #[must_use]
    pub unsafe fn new(addr: NonNull<()>) -> Self {
        Self {
            addr,
            pd: PhantomData,
        }
    }

    /// Returns an iterator of [`Atag`]. The first one should always be a [`AtagCore`] variant.
    #[must_use]
    pub fn iter(&'a self) -> AtagIter<'a> {
        AtagIter {
            _atags: self,
            addr: Some(self.addr),
        }
    }
}

/// Iterator returned from [`Atags`]. Yields [`Atag`] entries.
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
            let addr = pointer_offset(addr, core::mem::size_of::<AtagHeader>());

            match header.tag {
                0x5441_0001 => {
                    if header.size == 2 {
                        // empty core
                        None
                    } else {
                        Some(Atag::Core(unsafe { addr.cast().as_ref() }))
                    }
                }
                0x5441_0002 => Some(Atag::Memory(unsafe { addr.cast().as_ref() })),
                0x5441_0003 => Some(Atag::VideoText(unsafe { addr.cast().as_ref() })),
                0x5441_0004 => Some(Atag::RamDisk(unsafe { addr.cast().as_ref() })),
                0x5441_0005 => Some(Atag::InitRd2(unsafe { addr.cast().as_ref() })),
                0x5441_0006 => Some(Atag::Serial(unsafe { addr.cast().as_ref() })),
                0x5441_0007 => Some(Atag::Revision(unsafe { addr.cast().as_ref() })),
                0x5441_0008 => Some(Atag::VideoLfb(unsafe { addr.cast().as_ref() })),
                0x5441_0009 => Some(Atag::CommandLine {
                    cmdline: unsafe {
                        core::slice::from_raw_parts(addr.cast().as_ptr(), byte_length)
                    },
                }),
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
            (header.size as usize)
                .checked_mul(4)
                .map(|offset| pointer_offset(addr, offset))
        };
        result
    }
}

#[cfg(feature = "nightly")]
fn pointer_offset<T>(ptr: NonNull<T>, offset: usize) -> NonNull<T> {
    use core::num::NonZeroUsize;
    ptr.map_addr(|a| NonZeroUsize::new(a.get() + offset).unwrap())
}

#[cfg(not(feature = "nightly"))]
fn pointer_offset<T>(ptr: NonNull<T>, offset: usize) -> NonNull<T> {
    NonNull::new((ptr.as_ptr() as usize + offset) as *mut T).unwrap()
}

/// Determines which tag is in the given memory region.
#[derive(Debug)]
pub enum Atag<'a> {
    /// Start tag used to begin list.
    Core(&'a AtagCore),
    /// Tag used to describe a physical area of memory.
    Memory(&'a AtagMemory),
    /// Tag used to describe VGA text type displays
    VideoText(&'a AtagVideoText),
    /// Tag describing how the ramdisk will be used by the kernel
    RamDisk(&'a AtagRamDisk),
    /// Tag describing the physical location of the compressed ramdisk image
    InitRd2(&'a AtagInitRd2),
    /// Tag with 64 bit serial number of the board
    Serial(&'a AtagSerial),
    /// Tag for the board revision
    Revision(&'a AtagRevision),
    /// Tag describing parameters for a framebuffer type display
    VideoLfb(&'a AtagVideoLfb),
    /// Tag used to pass the commandline to the kernel
    CommandLine {
        /// Used to pass command line parameters to the kernel. The command line must be NULL terminated.
        cmdline: &'a [u8],
    },
    /// Unknown tag found. The content fields are the data read. If you encounter this variant, please open an issue.
    Unknown {
        /// The header that was read.
        header: &'a AtagHeader,
        /// The remaining data that was read.
        data: &'a [u8],
    },
}

/// A raw headera. Used for debugging.
#[repr(C)]
#[derive(Debug)]
pub struct AtagHeader {
    size: u32,
    #[debug(format = "0x{:08X}")]
    tag: u32,
}

/// This tag must be used to start the list, it contains the basic information any bootloader must pass.
#[repr(C)]
#[derive(Debug)]
pub struct AtagCore {
    /// bit 0 = read-only if high
    pub flags: u32,

    /// systems page size (usually 4k)
    #[debug(format = "0x{:04X}")]
    pub page_size: u32,

    /// Root device number
    #[debug(format = "0x{:08X}")]
    pub root_device_number: u32,
}

/// Describes an area of physical memory the kernel is to use.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AtagMemory {
    /// The size of the area.
    #[debug(format = "0x{:08X}")]
    pub size: u32,
    /// Physical start address.
    #[debug(format = "0x{:08X}")]
    pub start: u32,
}

/// Tag used to describe VGA text type displays.
#[repr(C)]
#[derive(Debug)]
pub struct AtagVideoText {
    /// Width of display
    pub width: u8,
    /// Height of display
    pub height: u8,
    ///
    pub video_page: u16,
    ///
    pub video_mode: u8,
    ///
    pub video_cols: u8,
    ///
    pub video_ega_bx: u16,
    ///
    pub video_lines: u8,
    ///
    pub video_isvga: u8,
    ///
    pub video_points: u16,
}

/// Describes how the (initial) ramdisk will be configured by the kernel, specifically this allows for the bootloader to
/// ensure the ramdisk will be large enough to take the decompressed initial ramdisk image the bootloader is passing
/// using [`AtagInitRd2`]
#[repr(C)]
#[derive(Debug)]
pub struct AtagRamDisk {
    /// bit 0 = load, bit 1 = prompt
    pub flags: u32,
    /// decompressed ramdisk size in _kilo_ bytes
    #[debug(format = "{} kb")]
    pub size: u32,
    /// starting block of floppy-based RAM disk image
    #[debug(format = "0x{:08X}")]
    pub start: u32,
}

/// Location of a compressed ramdisk image, usually combined with an [`AtagRamDisk`]. Can be used as an initial root file
/// system with the addition of a command line parameter of 'root=/dev/ram'. This tag supersedes the original
/// `ATAG_INITRD` which used virtual addressing, this was a mistake and produced issues on some systems. All new
/// bootloaders should use this tag in preference.
#[repr(C)]
#[derive(Debug)]
pub struct AtagInitRd2 {
    /// Physical start address
    #[debug(format = "0x{:08X}")]
    pub start: u32,
    /// size of compressed ramdisk image in bytes
    #[debug(format = "0x{:08X}")]
    pub size: u32,
}

/// Tag with 64 bit serial number of the board
#[repr(C)]
#[derive(Debug)]
pub struct AtagSerial {
    ///
    pub low: u32,
    ///
    pub high: u32,
}

/// Tag for the board revision
#[repr(C)]
#[derive(Debug)]
pub struct AtagRevision {
    ///
    pub revision: u32,
}

/// Tag describing parameters for a framebuffer type display
#[repr(C)]
#[derive(Debug)]
pub struct AtagVideoLfb {
    ///
    pub width: u16,
    ///
    pub height: u16,
    ///
    pub depth: u16,
    ///
    pub line_length: u16,
    ///
    pub base: u32,
    ///
    pub size: u32,
    ///
    pub red_size: u8,
    ///
    pub red_pos: u8,
    ///
    pub green_size: u8,
    ///
    pub green_pos: u8,
    ///
    pub blue_size: u8,
    ///
    pub blue_pos: u8,
    ///
    pub rsvd_size: u8,
    ///
    pub rsvd_pos: u8,
}
