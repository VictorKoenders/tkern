use cortex_a::registers::CurrentEL;
use tock_registers::interfaces::Readable;

extern "C" {
    // Defined in linker.ld
    static __kernel_start: u64;
    static __kernel_end: u64;
}

/// Get the start address of the kernel (always going to be `0x80000`)
pub fn kernel_start() -> usize {
    unsafe { core::ptr::addr_of!(__kernel_start).addr() }
}

/// Get the end address of the kernel, 8-byte aligned
pub fn kernel_end() -> usize {
    unsafe { core::ptr::addr_of!(__kernel_end).addr() }
}

pub fn current_exception_level() -> Exception {
    let register = CurrentEL.read_as_enum(CurrentEL::EL);
    match register {
        Some(CurrentEL::EL::Value::EL0) => Exception::Level0,
        Some(CurrentEL::EL::Value::EL1) => Exception::Level1,
        Some(CurrentEL::EL::Value::EL2) => Exception::Level2,
        Some(CurrentEL::EL::Value::EL3) => Exception::Level3,
        _ => unreachable!(),
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Exception {
    Level0,
    Level1,
    Level2,
    Level3,
}

/// Transition from EL2 (hypervisor) to EL1 (OS kernel)
///
/// # Safety
///
/// This function must be called when in EL2 mode, and must be called only once.
///
/// The `bss_address` must point to a valid stack address that the kernel can use.
///
/// The `target_fn` must point to a valid kernel entry function.
pub unsafe fn transition_from_el2_to_el1(bss_address: u64, target_fn: extern "C" fn() -> !) -> ! {
    use cortex_a::{
        asm,
        registers::{CNTHCTL_EL2, CNTVOFF_EL2, ELR_EL2, HCR_EL2, SPSR_EL2, SP_EL1},
    };
    use tock_registers::interfaces::Writeable;

    // Enable timer counter registers for EL1.
    CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);
    CNTVOFF_EL2.set(0);
    HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);
    SPSR_EL2.write(
        SPSR_EL2::D::Masked
            + SPSR_EL2::A::Masked
            + SPSR_EL2::I::Masked
            + SPSR_EL2::F::Masked
            + SPSR_EL2::M::EL1h,
    );
    ELR_EL2.set(target_fn as *const () as u64);
    SP_EL1.set(bss_address);
    asm::eret();
}

/// This is called from `src/boot.s`. See that file for more info
#[no_mangle]
pub extern "C" fn _start_rust(
    atag_addr: u64, // x0: 32 bit pointer to atag in memory (primary core only) / 0 (secondary cores)
    _x1: u64,       // always 0 for now
    _x2: u64,       // always 0 for now
    _x3: u64,       // always 0 for now
    start_address: u64, // x4: The start address on which the kernel started. This will be the `_start` label in our `boot.s`
) -> ! {
    unsafe {
        crate::START_ADDRESS.set(start_address);
        crate::ATAG_ADDR.set(atag_addr);

        transition_from_el2_to_el1(start_address, crate::kernel_main);
    }
}
