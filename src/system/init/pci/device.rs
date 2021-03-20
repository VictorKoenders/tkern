use super::{read_location_u16, read_location_u32, read_location_u8, BaseAddress, Location};
use bitflags::bitflags;

/// Information regarding a device that was found on the PCI bus.
#[derive(Debug)]
pub struct Device {
    /// Identifier information of the device
    pub id: DeviceId,
    /// The kind of this device
    pub kind: DeviceKind,
    /// The commands this device can accept
    pub command: Command,
    /// The status of the device
    pub status: Status,

    /// `true` if the device supports multiple functions
    pub multiple_functions: bool,
    /// The location of the device
    pub location: Location,
}

impl Device {
    pub(super) fn read(vendor_id: u16, device_id: u16, location: Location) -> Option<Self> {
        // For more information about the memory layout of a PCI device, see
        // https://wiki.osdev.org/PCI#PCI_Device_Structure
        let (command, status) = read_location_u16(location.with_offset(4));
        let command = Command::from_bits_truncate(command);
        let status = Status::from_bits_truncate(status);

        let (id, header_type) = DeviceId::read(vendor_id, device_id, location)?;
        // header_type bit 7 is used to indicate that this device has multiple functions
        let multiple_functions = (header_type & 0x80) > 0;
        let kind = DeviceKind::read(header_type & 0x7F, location)?;

        Some(Device {
            id,
            kind,
            command,
            status,

            multiple_functions,
            location,
        })
    }
}

/// Contains information about the identification of the device.
///
/// For information on a specific device, you can add the HEX value of the following fields: (with rust formatting)
///
/// https://pci-ids.ucw.cz/read/PC/{vendor:04X}/{device:04X}/
#[derive(Debug)]
pub struct DeviceId {
    /// Identifies the manufacturer of the device.
    /// Where valid IDs are allocated by PCI-SIG (the list is here) to ensure uniqueness and 0xFFFF is an invalid value that will be returned on read accesses to Configuration Space registers of non-existent devices.
    pub vendor: u16,
    /// Identifies the particular device. Where valid IDs are allocated by the vendor.
    pub device: u16,
    /// Specifies a revision identifier for a particular device. Where valid IDs are allocated by the vendor.
    pub revision_id: u8,
    /// A read-only register that specifies a register-level programming interface the device has, if it has any at all.
    pub programmable_interface: u8,
    /// A read-only register that specifies the type of function the device performs.
    pub class: DeviceClass,
    /// A read-only register that specifies the specific function the device performs.
    pub subclass: u8,
    /// Specifies the system cache line size in 32-bit units.
    /// A device can limit the number of cacheline sizes it can support, if a unsupported value is written to this field,
    /// the device will behave as if a value of 0 was written.
    pub cache_line_size: u8,
    /// Specifies the latency timer in units of PCI bus clocks.
    pub latency_timer: u8,
    /// Represents that status and allows control of a devices BIST (built-in self test)
    pub build_in_self_test: u8,
}

impl DeviceId {
    pub(super) fn read(vendor: u16, device: u16, location: Location) -> Option<(Self, u8)> {
        let (revision_id, programmable_interface, subclass, class_id) =
            read_location_u8(location.with_offset(0x08));

        let (cache_line_size, latency_timer, header_type, build_in_self_test) =
            read_location_u8(location.with_offset(0x0C));

        Some((
            Self {
                vendor,
                device,
                revision_id,
                programmable_interface,
                class: DeviceClass::new(class_id),
                subclass,

                cache_line_size,
                latency_timer,
                build_in_self_test,
            },
            header_type,
        ))
    }
}

numeric_enum! {
    /// The Class Code, Subclass, and Prog IF registers are used to identify the device's type, the device's function, and the device's register-level programming interface, respectively.
    #[derive(Debug, PartialEq, Eq)]
    pub enum DeviceClass: u8 {
        ///
        Unclassified = 0x00,
        ///
        MassStorageController = 0x01,
        ///
        NetworkController = 0x02,
        ///
        DisplayController = 0x03,
        ///
        MultimediaController = 0x04,
        ///
        MemoryController = 0x05,
        ///
        BridgeDevice = 0x06,
        ///
        SimpleCommunicationController = 0x07,
        ///
        BaseSystemPeripheral = 0x08,
        ///
        InputDeviceController = 0x09,
        ///
        DockingStation = 0x0A,
        ///
        Processor = 0x0B,
        ///
        SerialBusController = 0x0C,
        ///
        WirelessController = 0x0D,
        ///
        IntelligentController = 0x0E,
        ///
        SatelliteCommunicationController = 0x0F,
        ///
        EncryptionController = 0x10,
        ///
        SignalProcessingController = 0x11,
        ///
        ProcessingAcceleration = 0x12,
        ///
        NonEssentialInstrumentation = 0x13,
        ///
        CoProcessor = 0x40,
        ///
        Unassigned = 0xFF
    }
}

/// An enum with the different types of devices that this kernel supports
#[derive(Debug)]
#[non_exhaustive]
pub enum DeviceKind {
    /// General device information
    General(GeneralDevice),
}

impl DeviceKind {
    pub(super) fn read(header_type: u8, location: Location) -> Option<Self> {
        match header_type {
            0x00 => GeneralDevice::read(location).map(DeviceKind::General),
            _ => panic!("Unknown header type 0x{:02X}", header_type),
        }
    }
}

/// General device
#[derive(Debug)]
pub struct GeneralDevice {
    /// Base Address Registers (or BARs) can be used to hold memory addresses used by the device, or offsets for port addresses.
    /// Typically, memory address BARs need to be located in physical ram while I/O space BARs can reside at any memory address (even beyond physical memory).
    pub bars: [BaseAddress; 6],
    /// Points to the Card Information Structure and is used by devices that share silicon between CardBus and PCI.
    pub cardbus_cis_ptr: u32,
    /// The ID of the subsystem
    pub subsystem_id: u16,
    /// The vendor of the subsystem
    pub subsystem_vendor_id: u16,
    ///
    pub expansion_rom_base_address: u32,
    /// Points (i.e. an offset into this function's configuration space) to a linked list of new capabilities implemented by the device.
    /// Used if bit 4 of the status register (Capabilities List bit) is set to 1.
    /// The bottom two bits are reserved and should be masked before the Pointer is used to access the Configuration Space.
    pub capabilities: u8,
    /// A read-only register that specifies how often the device needs access to the PCI bus (in 1/4 microsecond units).
    pub max_latency: u8,
    /// A read-only register that specifies the burst period length, in 1/4 microsecond units, that the device needs (assuming a 33 MHz clock rate).
    pub min_grant: u8,
    /// Specifies which interrupt pin the device uses.
    /// Where a value of 0x01 is INTA#, 0x02 is INTB#, 0x03 is INTC#, 0x04 is INTD#, and 0x00 means the device does not use an interrupt pin.
    pub interrupt_pin: u8,
    /// Specifies which input of the system interrupt controllers the device's interrupt pin is connected to and is implemented by any device that makes use of an interrupt pin.
    /// For the x86 architecture this register corresponds to the PIC IRQ numbers 0-15 (and not I/O APIC IRQ numbers) and a value of 0xFF defines no connection.
    pub interrupt_line: u8,
}

impl GeneralDevice {
    fn read(location: Location) -> Option<GeneralDevice> {
        use core::mem::{transmute, MaybeUninit};

        // Safety: example code from https://doc.rust-lang.org/stable/core/mem/union.MaybeUninit.html
        let mut bars: [MaybeUninit<BaseAddress>; 6] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for (index, bar) in bars.iter_mut().enumerate() {
            let index = ((index * 4) + 0x10) as u8;
            *bar = MaybeUninit::new(BaseAddress::read(location.with_offset(index))?);
        }
        let bars: [BaseAddress; 6] = unsafe { transmute(bars) };

        let cardbus_cis_ptr = read_location_u32(location.with_offset(0x28));
        let (subsystem_id, subsystem_vendor_id) = read_location_u16(location.with_offset(0x2C));
        let expansion_rom_base_address = read_location_u32(location.with_offset(0x30));
        let (_, _, _, capabilities) = read_location_u8(location.with_offset(0x34));
        let (max_latency, min_grant, interrupt_pin, interrupt_line) =
            read_location_u8(location.with_offset(0x3C));

        Some(GeneralDevice {
            bars,
            cardbus_cis_ptr,
            subsystem_id,
            subsystem_vendor_id,
            expansion_rom_base_address,
            capabilities,
            max_latency,
            min_grant,
            interrupt_pin,
            interrupt_line,
        })
    }

    /// Get a human readable name for a given DeviceKind and DeviceId combination, if known by the kernel.
    pub fn get_known_name(&self, id: &DeviceId) -> Option<&'static str> {
        match (
            id.vendor,
            id.device,
            self.subsystem_id,
            self.subsystem_vendor_id,
        ) {
            (0x8086, 0x10D3, _, _) => Some("Gigabit Ethernet Controller"),
            (0x8086, 0x1237, _, _) => Some("PCI and memory controller"),
            (0x8086, 0x2918, _, _) => Some("Low-Pin-Count (legacy) controller"),
            (0x8086, 0x2922, _, _) => Some("6 port SATA controller"),
            (0x8086, 0x2930, _, _) => Some("System management bus"),
            (0x8086, 0x29c0, _, _) => Some("DRAM Controller"),
            (0x8086, 0x7000, _, _) => Some("Intel 82371 PIIX3 controller"),
            (0x8086, 0x7010, _, _) => Some("Intel 82371 PIIX3 controller"),
            (0x8086, 0x7113, _, _) => Some("Intel 82371 PIIX4 controller"),
            (0x1b36, 0x0100, _, _) => Some("QXL paravirtual graphic card"),
            (0x8086, 0x100E, _, _) => Some("82540EM Gigabit Ethernet Controller "),
            _ => None,
        }
    }
}

bitflags! {
    /// Provides control over a device's ability to generate and respond to PCI cycles.
    /// Where the only functionality guaranteed to be supported by all devices is,
    /// when a 0 is written to this register,
    /// the device is disconnected from the PCI bus for all accesses except Configuration Space access.
    pub struct Command: u16 {
        /// If set the device can respond to I/O Space accesses; otherwise, the device's response is disabled.
        const IO_SPACE                  = 0x0001;
        /// If set the device can respond to Memory Space accesses; otherwise, the device's response is disabled.
        const MEMORY_SPACE              = 0x0002;
        /// If set the device can behave as a bus master; otherwise, the device can not generate PCI accesses.
        const BUS_MASTER                = 0x0004;
        /// If set the device can monitor Special Cycle operations; otherwise, the device will ignore them.
        const SPECIAL_CYCLES            = 0x0008;
        /// If set the device can generate the Memory Write and Invalidate command; otherwise,
        /// the Memory Write command must be used.
        const MWI_ENABLE                = 0x0010;
        /// If set the device does not respond to palette register writes and will snoop the data; otherwise,
        /// the device will trate palette write accesses like all other accesses.
        const VGA_PALETTE_SNOOP         = 0x0020;
        /// If set the device will take its normal action when a parity error is detected; otherwise,
        /// when an error is detected, the device will set bit 15 of the Status register (Detected Parity Error Status Bit),
        /// but will not assert the PERR# (Parity Error) pin and will continue operation as normal.
        const PARITY_ERROR_RESPONSE     = 0x0040;
        /// If set the SERR# driver is enabled; otherwise, the driver is disabled.
        const SERR_ENABLE               = 0x0100;
        /// If set indicates a device is allowed to generate fast back-to-back transactions; otherwise, fast back-to-back transactions are only allowed to the same agent.
        const FAST_BACK_TO_BACK_ENABLE  = 0x0200;
        /// If set the assertion of the devices INTx# signal is disabled; otherwise, assertion of the signal is enabled.
        const INTERRUPT_DISABLE         = 0x0400;
    }
}

bitflags! {
    /// A register used to record status information for PCI bus related events.
    pub struct Status: u16 {
        /// Represents the state of the device's INTx# signal. If set and bit 10 of the Command register (Interrupt Disable bit) is set to 0 the signal will be asserted; otherwise, the signal will be ignored.
        const INTERRUPT_STATUS          = 0x0008;
        /// If set the device implements the pointer for a New Capabilities Linked list at offset 0x34; otherwise,
        /// the linked list is not available.
        const CAPABILITIES_LIST         = 0x0010;
        /// If set the device is capable of running at 66 MHz; otherwise, the device runs at 33 MHz.
        const MHZ66_CAPABLE             = 0x0020;
        /// If set the device can accept fast back-to-back transactions that are not from the same agent; otherwise,
        /// transactions can only be accepted from the same agent.
        const FAST_BACK_TO_BACK_CAPABLE = 0x0080;
        /// This bit is only set when the following conditions are met.
        /// The bus agent asserted PERR# on a read or observed an assertion of PERR# on a write,
        /// the agent setting the bit acted as the bus master for the operation in which the error occurred,
        /// and bit 6 of the Command register (Parity Error Response bit) is set
        const MASTER_DATA_PARITY_ERROR  = 0x0100;
        /// Read only bits that represent the slowest time that a device will assert DEVSEL# for any bus command except Configuration Space read and writes.
        const DEVSEL_MEDIUM_TIMING      = 0x0200;
        /// Read only bits that represent the slowest time that a device will assert DEVSEL# for any bus command except Configuration Space read and writes.
        const DEVSEL_SLOW_TIMING        = 0x0400;
        /// This bit will be set whenever a target device terminates a transaction with Target-Abort.
        const SIGNALED_TARGET_ABORT     = 0x0800;
        /// This bit will be set  by a master device, whenever its transaction is terminated with Target-Abort.
        const RECEIVED_TARGET_ABORT     = 0x1000;
        /// This bit will be set  by a master device, whenever its transaction (except for Special Cycle transactions) is terminated with Master-Abort.
        const RECEIVED_MASTER_ABORT     = 0x2000;
        /// This bit will be set whenever the device asserts SERR#.
        const SIGNALED_SYSTEM_ERROR     = 0x4000;
        /// This bit will be set whenever the device detects a parity error, even if parity error handling is disabled.
        const DETECTED_PARITY_ERROR     = 0x8000;
    }
}
