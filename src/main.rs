#![no_std]
#![no_main]

mod logger;

use bootloader::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(fb) = boot_info.framebuffer.as_mut() {
        logger::init(fb);
    }
    log::info!("Hello from kernel");
    log::info!("Y: {}", logger::get(|l| l.y_pos));
    log::info!("Y: {}", logger::get(|l| l.y_pos));

    loop {
        log::trace!("Hello from kernel");
        delay();
        log::debug!("Hello from kernel");
        delay();
        log::warn!("Hello from kernel");
        delay();
        log::error!("Hello from kernel");
        delay();
    }
}

fn delay() {
    for _ in 0..1_000_000 {
        x86_64::instructions::nop();
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
