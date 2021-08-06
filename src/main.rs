#![no_std]
#![no_main]

mod logger;

use bootloader::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(fb) = boot_info.framebuffer.as_mut() {
        logger::init(fb);
    }

    for _ in 0..9 {
        log::trace!("Hello from kernel");
        log::debug!("Hello from kernel");
        log::info!("Hello from kernel");
        log::warn!("Hello from kernel");
        log::error!("Hello from kernel");
    }
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
