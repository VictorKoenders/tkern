use super::{Dsdt, Header};
use crate::memory::PhysicalAddress;

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct V1 {
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
    pub const LENGTH: u32 = core::mem::size_of::<V1>() as u32;

    pub fn dsdt<'a>(&self, allocator: &'a super::TableAllocator) -> &'a Dsdt {
        unsafe { allocator.get_table_known_type(PhysicalAddress(self.dsdt_address as u64)) }
    }
}

fn _test_size() {
    unsafe {
        core::mem::transmute::<[u8; 116], V1>([0u8; 116]);
        // core::mem::transmute::<[u8; 12], GenericAddressStruct>([0u8; 12]);
    }
}

/*
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
pub struct GenericAddressStruct {
    address_space: u8,
    bit_width: u8,
    bit_offset: u8,
    access_size: u8,
    address: u64,
}
*/
