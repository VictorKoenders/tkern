use crate::memory::VirtualAddress;
use x86_64::{
    instructions::{hlt, interrupts::without_interrupts as without_interrupts_inner, tlb},
    VirtAddr,
};

/// Flush the virtual memory table lookahead cache. This is called whenever [Memory] frees a frame.
pub fn flush_tlb(address: VirtualAddress) {
    tlb::flush(VirtAddr::new(address.0 as u64));
}

/// Run the given callback in a context where interrupts won't be triggered.
///
/// This is useful for small pieces of code that cannot afford to be interrupted, e.g. for mutex locking code.
pub fn without_interrupts<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    without_interrupts_inner(f)
}

/// Loop the current CPU indefinitely with halt instructions.
/// Using this method will put the CPU in a semi-sleep state, and should be preferred over empty `loop {}` blocks.
pub fn halt_loop() -> ! {
    loop {
        hlt();
    }
}
