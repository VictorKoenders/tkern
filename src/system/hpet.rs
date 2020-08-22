use super::Header;

/// High precision event timer.
///
/// https://wiki.osdev.org/HPET
#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct Hpet {
    /// Common SDT header
    pub header: Header,
    hardware_rev_id: u8,
    count: Count,
    pci_vendor_id: u16,
    address_space_id: u8,
    address_register_bit_width: u8,
    address_register_bit_offset: u8,
    address_reserved: u8,
    address: u64,
    hpet_number: u8,
    minimum_tick: u16,
    page_protection: u8,
}

/// Compressed struct for `comparitor`, `counter_size` and `legacy_replacement`
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Count(u8);

impl Count {
    /// comparitor
    pub fn comparator(&self) -> u8 {
        (self.0 & 0b1111_1000) >> 3
    }
    /// Counter size
    pub fn counter_size(&self) -> u8 {
        (self.0 & 0b0000_0100) >> 2
    }
    /// if true, HPET's timer (comparator) #0 replaces PIT interrupts, whereas timer #1 replaces RTC's interrupts
    pub fn legacy_replacement(&self) -> bool {
        self.0 & 0b0000_0001 > 0
    }
}

fn _test_size() {
    unsafe {
        core::mem::transmute::<[u8; 56], Hpet>([0u8; 56]);
    }
}
