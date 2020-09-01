use core::fmt;

/// Indicates a location on the PCI bus
#[derive(Copy, Clone)]
pub struct Location(pub(super) u32);

impl Default for Location {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            fmt,
            "Location {{ bus: {:X}, device: {:X}, function: {:X}, offset: {:X} }}",
            self.bus(),
            self.device(),
            self.function(),
            self.offset()
        )
    }
}

macro_rules! bitfield {
    ($(#[$get_attr:meta])*  $get:ident, $(#[$set_attr:meta])* $set:ident, $offset:expr, $mask:expr) => {
        $(#[$get_attr])*
        pub fn $get(&self) -> u8 {
            let offset: usize = $offset;
            let mask: u32 = $mask;
            ((self.0 >> offset) & mask) as u8
        }

        $(#[$set_attr])*
        pub fn $set(&mut self, val: u8) {
            let offset: usize = $offset;
            let mask: u8 = $mask;
            let inverse_mask: u32 = (!(mask as u32)) << offset;

            assert!(
                (val & !mask) == 0,
                "{} (0b{:08b}) is invalid for {:?}, should be max 0b{:08b}",
                val,
                val,
                stringify!($get),
                mask
            );
            self.0 = (self.0 & inverse_mask) | ((val as u32) << offset)
        }
    };
}

impl Location {
    /// Create a new location with the given values.
    ///
    /// Will panic when the given values are invalid:
    /// - Device must be 0..=31
    /// - Function must be 0..=7
    /// - Offset must be a multiple of 4 (end in 2 0-bits)
    pub fn new(bus: u8, device: u8, function: u8, offset: u8) -> Self {
        let mut loc = Self(1 << 31); // bit 31 should always be high
        loc.set_bus(bus);
        loc.set_device(device);
        loc.set_function(function);
        loc.set_offset(offset);
        loc
    }

    /// Get the next location.
    ///
    /// This will iterate through the following fields in order:
    /// - Function: 0..=7
    /// - Device: 0..=31
    /// - Bus: 0..=255
    ///
    /// After `Function` reaches 8, it will reset to 0 and `Device` will be incremented, etc.
    pub fn next(self) -> Option<Location> {
        // The relevant location is stored from bits 8 to 24
        // Relevant meaning: function, device and bus
        // This is a continuous bitrange so we can just add 1 to the function
        // and check if the bus overflows
        let mut inner = self.0;
        inner += 1 << 8; // function is at bit 8
        if (inner & (1 << 24)) > 0 {
            // we went past the largest bus value
            None
        } else {
            // valid location, return it
            Some(Self(inner))
        }
    }

    #[must_use]
    /// Returns a location which is a copy of this location, but with the given offset instead.
    pub fn with_offset(mut self, offset: u8) -> Self {
        self.set_offset(offset);
        self
    }

    bitfield!(
        /// Get the bus location that this Location is pointing at
        bus,
        /// Set the bus location that this Location is pointing at
        set_bus,
        16,
        0b11111111
    );
    bitfield!(
        /// Get the device location that this Location is pointing at
        device,
        /// Set the device location that this Location is pointing at
        ///
        /// Only values 0..31 is valid. When passing a value of 32 or higher to this function, this function will panic.
        set_device,
        11,
        0b11111
    );
    bitfield!(
        /// Get the function location that this Location is pointing at
        function,
        /// Set the function location that this Location is pointing at
        ///
        /// Only values 0..7 is valid. When passing a value of 8 or higher to this function, this function will panic.
        set_function,
        8,
        0b111
    );
    bitfield!(
        /// Get the offset location that this Location is pointing at
        offset,
        /// Set the offset location that this Location is pointing at
        ///
        /// The offset must be 4-byte aligned. In other words, the last 2 bits must be zero.
        /// If this is not the case, this function will panic
        set_offset,
        0,
        0b11111100
    );
}

#[cfg(test)]
mod test {
    use super::Location;
    #[test]
    fn validate_mapping() {
        for bus in 0..=255 {
            for device in 0..=0b11111 {
                for function in 0..=0b111 {
                    for offset in (0..=255).step_by(4) {
                        let location = Location::new(bus, device, function, offset);
                        assert_eq!(location.bus(), bus);
                        assert_eq!(location.device(), device);
                        assert_eq!(location.function(), function);
                        assert_eq!(location.offset(), offset);
                    }
                }
            }
        }
        let mut loc = Location::new(0, 0, 0, 0);
        for bus in 0..=255 {
            loc.set_bus(bus);
            for device in 0..=0b11111 {
                loc.set_device(device);
                for function in 0..=0b111 {
                    loc.set_function(function);
                    for offset in (0..=255).step_by(4) {
                        loc.set_offset(offset);

                        assert_eq!(loc.bus(), bus);
                        assert_eq!(loc.device(), device);
                        assert_eq!(loc.function(), function);
                        assert_eq!(loc.offset(), offset);
                    }
                }
            }
        }
    }
    #[test]
    #[should_panic]
    fn invalid_device_should_panic() {
        Location::new(0, 0b100000, 0, 0);
    }
    #[test]
    #[should_panic]
    fn invalid_function_should_panic() {
        Location::new(0, 0, 0b1000, 0);
    }
    #[test]
    #[should_panic]
    fn invalid_offset_should_panic_1() {
        Location::new(0, 0, 0, 1);
    }
    #[test]
    #[should_panic]
    fn invalid_offset_should_panic_2() {
        Location::new(0, 0, 0, 2);
    }
    #[test]
    #[should_panic]
    fn invalid_offset_should_panic_3() {
        Location::new(0, 0, 0, 3);
    }
    #[test]
    fn offset_4_is_fine() {
        Location::new(0, 0, 0, 4);
    }
}
