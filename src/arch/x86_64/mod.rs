use crate::memory::VirtualAddress;
use x86_64::{
    instructions::{hlt, interrupts::without_interrupts as without_interrupts_inner, tlb},
    VirtAddr,
};

pub fn flush_tlb(address: VirtualAddress) {
    tlb::flush(VirtAddr::new(address.0 as u64));
}

pub fn without_interrupts<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    without_interrupts_inner(f)
}

pub fn halt_loop() -> ! {
    loop {
        hlt();
    }
}
