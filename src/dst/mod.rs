use core::{fmt};
use utils::const_non_null::ConstNonNull;

#[derive(Debug)]
#[repr(C)]
pub struct Header {
    pub magic: BigEndianU32,
    pub totalsize: BigEndianU32,
    pub off_dt_struct: BigEndianU32,
    pub off_dt_strings: BigEndianU32,
    pub off_mem_rsvmap: BigEndianU32,
    pub version: BigEndianU32,
    pub last_comp_version: BigEndianU32,
    pub boot_cpuid_phys: BigEndianU32,
    pub size_dt_strings: BigEndianU32,
    pub size_dt_struct: BigEndianU32,
}

impl Header {
    /// Load the DST header from memory.
    /// 
    /// # Safety
    /// 
    /// The caller must ensure the given address is a valid memory address.
    /// This can be checked with reading a [`BigEndianU32`] at the given address, and checking if the value is `0xd00dfeed`.
    pub unsafe fn from_memory(address: ConstNonNull<()>) -> &'static Header {
        unsafe { address.cast().as_ref() }
    }
}

#[repr(transparent)]
pub struct BigEndianU32([u8; 4]);

impl fmt::Debug for BigEndianU32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", u32::from_be_bytes(self.0))
    }
}


impl BigEndianU32 {
    /// Load a BigEndianU32 from memory.
    /// 
    /// # Safety
    /// 
    /// The caller must ensure the given address is a valid memory address.
    pub unsafe fn from_memory(address: ConstNonNull<()>) -> &'static BigEndianU32 {
        unsafe { address.cast().as_ref() }
    }

    pub fn is_magic_header(&self) -> bool {
        u32::from_be_bytes(self.0) == 0xd00dfeed
    }
}
