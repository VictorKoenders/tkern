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
