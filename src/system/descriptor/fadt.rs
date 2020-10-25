#![allow(missing_docs, non_snake_case)]

use super::{Dsdt, Header};
use crate::memory::{PhysicalAddress, VirtualAddress};

#[derive(Debug, Clone)]
pub enum Fadt<'a> {
    V1(&'a V1),
    V2(&'a V2),
    V3(&'a V3),
}

impl Fadt<'_> {
    pub fn header(&self) -> &Header {
        match self {
            Fadt::V1(v1) => unsafe { &v1.header },
            Fadt::V2(v2) => unsafe { &v2.v1.header },
            Fadt::V3(v3) => unsafe { &v3.v2.v1.header },
        }
    }
    pub fn virtual_address(&self) -> VirtualAddress {
        match self {
            Fadt::V1(v1) => VirtualAddress::from_ref(*v1),
            Fadt::V2(v2) => VirtualAddress::from_ref(*v2),
            Fadt::V3(v3) => VirtualAddress::from_ref(*v3),
        }
    }
    pub fn mem_size(&self) -> usize {
        use core::mem::size_of;
        match self {
            Fadt::V1(_) => size_of::<V1>(),
            Fadt::V2(_) => size_of::<V2>(),
            Fadt::V3(_) => size_of::<V3>(),
        }
    }
    /// Get the DSDT table for this FADT entry
    pub fn dsdt<'a>(&self, allocator: &'a super::TableAllocator) -> &'a Dsdt {
        let address = match self {
            Fadt::V1(v1) => v1.dsdt_address,
            Fadt::V2(v2) => v2.v1.dsdt_address,
            Fadt::V3(v3) => v3.v2.v1.dsdt_address,
        };
        unsafe { allocator.get_table_known_type(PhysicalAddress(address as u64)) }
    }
}

/// FADT V1 struct. Contains information about fixed register blocks.
///
/// https://wiki.osdev.org/FADT
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct V1 {
    /// Common SDT header
    pub header: Header,
    facs_address: u32,
    dsdt_address: u32,
    reserved: u8,
    preferred_power_management_profile: u8,
    sci_interrupt: u16,
    smi_command_port: u32,
    acpi_enable: u8,
    acpi_disable: u8,
    s4bios_req: u8,
    pstate_control: u8,
    pm1a_event_block: u32,
    pm1b_event_block: u32,
    pm1a_control_block: u32,
    pm1b_control_block: u32,
    pm2_control_block: u32,
    pm_timer_block: u32,
    gpe0_block: u32,
    gpe1_block: u32,
    pm1_event_length: u8,
    pm1_control_length: u8,
    pm2_control_length: u8,
    pm_timer_length: u8,
    gpe0_length: u8,
    gpe1_length: u8,
    gpe1_base: u8,
    cstate_control: u8,
    worst_c2_latency: u16,
    worst_c3_latency: u16,
    flush_size: u16,
    flush_stride: u16,
    duty_offset: u8,
    duty_width: u8,
    day_alarm: u8,
    month_alarm: u8,
    century: u8,
    boot_architecture_flags: u16,
    reserved_2: u8,
    flags: u32,
}

impl V1 {
    /// The length of the FADT V1 struct
    pub const LENGTH: u32 = core::mem::size_of::<V1>() as u32;
}

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct V2 {
    pub v1: V1,

    // 12 byte structure; see below for details
    ResetReg: GenericAddressStructure,

    ResetValue: u8,
    Reserved3: [u8; 3],
}

impl V2 {
    pub const LENGTH: u32 = core::mem::size_of::<V2>() as u32;
}

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct V3 {
    pub v2: V2,

    // 64bit pointers - Available on ACPI 2.0+
    X_FirmwareControl: u64,
    X_Dsdt: u64,

    X_PM1aEventBlock: GenericAddressStructure,
    X_PM1bEventBlock: GenericAddressStructure,
    X_PM1aControlBlock: GenericAddressStructure,
    X_PM1bControlBlock: GenericAddressStructure,
    X_PM2ControlBlock: GenericAddressStructure,
    X_PMTimerBlock: GenericAddressStructure,
    X_GPE0Block: GenericAddressStructure,
    X_GPE1: GenericAddressStructure,
}

impl V3 {
    pub const LENGTH: u32 = core::mem::size_of::<V3>() as u32;
}

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct GenericAddressStructure {
    AddressSpace: u8,
    BitWidth: u8,
    BitOffset: u8,
    AccessSize: u8,
    Address: u64,
}

fn _test_size() {
    unsafe {
        core::mem::transmute::<_, V1>([0u8; 116]);
        core::mem::transmute::<_, V2>([0u8; 132]);
        core::mem::transmute::<_, V3>([0u8; 244]);
        core::mem::transmute::<_, GenericAddressStructure>([0u8; 12]);
    }
}
