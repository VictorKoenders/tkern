use crate::memory::VirtualAddress;
use x86_64::{
    instructions::{hlt, tlb},
    VirtAddr,
};

pub mod interrupts;
pub mod port;

/// Flush the virtual memory table lookahead cache. This is called whenever [Mapper] frees a frame.
///
/// [Mapper]: memory/struct.Mapper.html
pub fn flush_tlb(address: VirtualAddress) {
    tlb::flush(VirtAddr::new(address.0 as u64));
}

/// Loop the current CPU indefinitely with halt instructions.
/// Using this method will put the CPU in a semi-sleep state, and should be preferred over empty `loop {}` blocks.
pub fn halt_loop() -> ! {
    loop {
        hlt();
    }
}

/// Put the CPU in sleep mode until the next interrupt is received
pub fn halt() {
    hlt();
}
