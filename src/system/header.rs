#[repr(C)]
#[derive(Debug)]
/// The common header of any System Descriptor Table
pub struct Header {
    signature: [u8; 4],
    /// The length of the header in bytes. This includes the header itself
    pub length: u32,
    /// The revision of this System function
    pub revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_id: [u8; 8],
    /// The OEM revision
    pub oem_revision: u32,
    /// The creator ID
    pub creator_id: u32,
    /// The creator revision
    pub creator_revision: u32,
}

impl Header {
    /// The signature of this header, or the raw byte array if it's not a valid `&str`
    pub fn signature(&self) -> Result<&str, &[u8; 4]> {
        core::str::from_utf8(&self.signature).map_err(|_| &self.signature)
    }

    /// The OEM ID of this header, or the raw byte array if it's not a valid `&str`
    pub fn oem_id(&self) -> Result<&str, &[u8; 6]> {
        core::str::from_utf8(&self.oem_id).map_err(|_| &self.oem_id)
    }

    /// The OEM table ID of this header, or the raw byte array if it's not a valid `&str`
    pub fn oem_table_id(&self) -> Result<&str, &[u8; 8]> {
        core::str::from_utf8(&self.oem_table_id).map_err(|_| &self.oem_table_id)
    }

    /// Checks if the header is valid based on the given checksum
    pub fn is_valid(&self) -> bool {
        let checksum = self.as_bytes().iter().fold(0u8, |a, b| a.wrapping_add(*b));
        checksum == 0
    }

    fn as_bytes(&self) -> &[u8] {
        let slice = self as *const _ as *const u8;
        unsafe { core::slice::from_raw_parts(slice, self.length as usize) }
    }

    pub(super) fn bytes_len_after_header(&self) -> usize {
        self.length as usize - core::mem::size_of::<Header>()
    }

    pub(super) fn ptr_after_header(&self) -> *const u8 {
        ((self as *const Header as usize) + core::mem::size_of::<Header>()) as *const u8
    }
}
