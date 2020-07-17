pub struct Identity([u16; 256]);

impl Default for Identity {
    fn default() -> Self {
        Self([0u16; 256])
    }
}

// reference: https://docs.microsoft.com/en-us/windows-hardware/drivers/ddi/ata/ns-ata-_identify_device_data
impl Identity {
    pub(super) fn fill(&mut self, bus: &super::Bus) {
        for b in &mut self.0[..] {
            let high = bus.read();
            let low = bus.read();
            use byteorder::{BigEndian, ByteOrder};
            *b = BigEndian::read_u16(&[high, low]);
        }
    }

    /// Indicates that the response was incomplete.
    pub fn response_incomplete(&self) -> bool {
        (self.0[0] & 0b00100000_00000000) > 0
    }

    /// Indicates when that the device is fixed.
    pub fn fixed_device(&self) -> bool {
        (self.0[0] & 0b00000010_00000000) > 0
    }

    /// Indicates that the media is removable.
    pub fn removable_media(&self) -> bool {
        (self.0[0] & 0b00000001_00000000) > 0
    }

    /// Indicates that the device is an ATA device
    pub fn is_ata_device(&self) -> bool {
        (self.0[0] & 0b00000000_00000001) > 0
    }

    /// Indicates the number of cylinders on the device.
    pub fn num_cylinders(&self) -> u16 {
        self.0[1]
    }

    pub fn specific_configuration(&self) -> u16 {
        self.0[2]
    }

    /// Number of logical heads on the device.
    pub fn num_heads(&self) -> u16 {
        self.0[3]
    }

    // 4-5: retired: This member is no longer used.

    /// Indicates the number of sectors per track.
    pub fn num_sectors_per_track(&self) -> u16 {
        self.0[6]
    }

    /// Contains the first ID of the device's vendor.
    pub fn vendor_unique_id_1(&self) -> &[u16; 3] {
        arrayref::array_ref!(&self.0, 7, 3)
    }
}
