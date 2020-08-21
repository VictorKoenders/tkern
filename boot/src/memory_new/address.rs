pub struct PhysicalAddress(pub u64);
pub struct VirtualAddress(pub u64);

impl VirtualAddress {
    pub(super) fn from_paging_indices(
        p4_index: usize,
        p3_index: usize,
        p2_index: usize,
        p1_index: usize,
    ) -> Self {
        // A virtual address is mapped as:
        // eeeeeeee_eeeeeeee_44444444_43333333_33222222_22211111_1111oooo_oooooooo
        //
        // e: sign extension, the top 16 bits must be the same as the highest bit of the p4 index, bits 63..=47
        // 4: p4 index, bits 47..=39
        // 3: p3 index, bits 38..=30
        // 2: p2 index, bits 29..=21
        // 1: p1 index, bits 20..=12
        // o: 12-bit offset inside the 4kb page, bits 11..=0

        // Most significant bit in the 9-bit segment
        const MSB_MASK: usize = 0b1000000;

        // the topmost 12 bits must be the same value as the highest bit in p4_index
        let sign_extension: u64 = if (p4_index & MSB_MASK) > 0 {
            0b11111111_11111111
        } else {
            0b00000000_00000000
        };

        // The bottommost 12 bits are the offset inside the table, we'll set this to 0
        let offset: u64 = 0b0000_00000000;

        // make sure each index is only 9 bits
        let p4_index = (p4_index & 0b00000001_11111111) as u64;
        let p3_index = (p3_index & 0b00000001_11111111) as u64;
        let p2_index = (p2_index & 0b00000001_11111111) as u64;
        let p1_index = (p1_index & 0b00000001_11111111) as u64;

        // bitshift values are taken from the explanation at the start of this function
        let val: u64 = (sign_extension << 47)
            | (p4_index << 39)
            | (p3_index << 30)
            | (p2_index << 21)
            | (p1_index << 12)
            | offset;

        VirtualAddress(val)
    }

    pub(super) fn p1_index(&self) -> usize {
        // A virtual address is mapped as:
        // eeeeeeee_eeeeeeee_44444444_43333333_33222222_22211111_1111oooo_oooooooo
        // see `from_paging_indices` for a full explanation

        const MASK: u64 = 0b00011111_11110000_00000000;
        const OFFSET: u64 = 12;
        let val = (self.0 & MASK) >> OFFSET;

        val as usize
    }
}

pub trait AddressAccess<T> {
    /// Write to a virtual address
    ///
    /// # Safety
    ///
    /// The caller must ensure that writing to this address is valid and does not cause UB.
    /// Anything can happen when writing to a memory address, beware!
    unsafe fn write(&mut self, val: T);

    /// Read from a virtual address
    ///
    /// # Safety
    ///
    /// The caller must ensure that reading from this address is valid and does not cause UB.
    /// Anything can happen when reading from a memory address, beware!
    ///
    /// And yes, the CPU can change or reset memory addresses on read.
    unsafe fn read(&self) -> T;
}

// Implements `AddressAccess<$t>` for VirtualAddress
//
// This is just a wrapper around `core::ptr::write_volatile` and `core::ptr::read_volatile`
macro_rules! implement_address_access_for_virtual_address {
    ($t:ty) => {
        impl AddressAccess<$t> for VirtualAddress {
            unsafe fn write(&mut self, val: $t) {
                core::ptr::write_volatile(self.0 as *mut $t, val);
            }

            unsafe fn read(&self) -> $t {
                core::ptr::read_volatile(self.0 as *const $t)
            }
        }
    };
}

implement_address_access_for_virtual_address!(u64);
