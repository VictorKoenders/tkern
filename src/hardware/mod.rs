use core::ptr::NonNull;
use cortex_a::registers::MIDR_EL1;
use tkern_derive_utils::RegEnum;
use tock_registers::interfaces::Readable;

pub fn detect() -> Hardware {
    let midr = MIDR_EL1.get();
    let implementer = Implementer::new(MIDR_EL1::Implementer.read(midr));
    #[allow(clippy::cast_possible_truncation)]
    let variant = MIDR_EL1::Variant.read(midr) as u8;
    let architecture = Architecture::new(MIDR_EL1::Architecture.read(midr));
    #[allow(clippy::cast_possible_truncation)]
    let partnum = MIDR_EL1::PartNum.read(midr) as u16;
    #[allow(clippy::cast_possible_truncation)]
    let revision = MIDR_EL1::PartNum.read(midr) as u8;

    let (name, mmio_base_address): (&'static str, usize) = match implementer {
        Implementer::Arm => match partnum {
            0xB76 => ("Rpi1", 0x2000_0000),
            0xC07 => ("Rpi2", 0x3F00_0000),
            0xD03 => ("Rpi3", 0x3F00_0000),
            0xD08 => ("Rpi4", 0x3F00_0000),
            _ => ("Unknown", 0x2000_0000),
        },
        _ => ("Unknown", 0x2000_0000),
    };

    Hardware {
        name,
        implementer,
        variant,
        architecture,
        partnum,
        revision,
        mmio_base_address: NonNull::new(mmio_base_address as _).expect("Unknown MMIO base address"),
    }
}

#[derive(RegEnum, Debug)]
pub enum Implementer {
    /// 0x00: Reserved for software use
    Reserved,

    /// 0x41: ARM Limited
    Arm,

    /// 0x42: Broadcom Corpotation
    Broadcom,

    /// 0x43: Cavium Inc
    Cavium,

    /// 0x44: Digital Equipment Corporation
    DigitalEquipment,

    /// 0x46: Fujitsu Ltd.
    Fujitsu,

    /// 0x49: Infineon Technologies AG.
    Infineon,

    /// 0x4D: Motorola or Freescale Semiconductor Inc.
    MotorolaOrFreescale,

    /// 0x4E: NVIDIA Corporation.
    Nvidia,

    /// 0x50: Applied Micro Circuits Corporation.
    AppliedMicroCircuits,

    /// 0x51: Qualcomm Inc
    Qualcomm,

    /// 0x56: Marvell International Ltd.
    Marvell,

    /// 0x69: Intel Corporation
    Intel,

    /// 0xC0: Ampere Computing
    Ampere,

    /// Unknown implementer
    Unknown(u8),
}

#[derive(RegEnum, Debug)]
pub enum Architecture {
    /// 0b0001: Armv4
    Armv4,

    /// 0b0010: Armv4T
    Armv4T,

    /// 0b0011: Armv5 (obsolete)
    Armv5,

    /// 0b0100: Armv5T
    Armv5T,

    /// 0b0101: Armv5TE
    Armv5TE,

    /// 0b0110: Armv5TEJ
    Armv5TEJ,

    /// 0b0111: Armv6
    Armv6,

    /// 0b1111: Architectural features are individually defined in the `ID_*` register
    IndividuallyDefined,

    Unknown(u8),
}
#[derive(custom_debug_derive::Debug)]
pub struct Hardware {
    pub name: &'static str,
    pub implementer: Implementer,
    #[debug(format = "0x{:02X}")]
    pub variant: u8,
    pub architecture: Architecture,
    #[debug(format = "0x{:X}")]
    pub partnum: u16,
    #[debug(format = "0x{:02X}")]
    pub revision: u8,
    pub mmio_base_address: NonNull<()>,
}
