use crate::memory_new::VirtualAddress;
use cortex_a::{
    asm,
    regs::{RegisterReadWrite, DAIF},
};

pub fn flush_tlb(address: VirtualAddress) {
    unsafe {
        llvm_asm!("mcr     p15, 0, $0, c8, c7, 0" :: "0"(address.0));
    }
}

pub fn without_interrupts<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    let d = DAIF::D::CLEAR.read(DAIF::D);
    let interrupts_were_enabled: bool = (d & DAIF::D::CLEAR.mask) > 0;

    if !interrupts_were_enabled {
        f()
    } else {
        DAIF::D::CLEAR.modify(1);
        let result = f();
        DAIF::D::SET.modify(1);

        result
    }
}

pub fn halt_loop() -> ! {
    loop {
        asm::wfi();
    }
}
