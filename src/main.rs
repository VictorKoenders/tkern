#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(kernel::test::runner))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use kernel::{
    allocator,
    platform::memory::{self, prelude::*},
    task::{executor::Executor, Task},
    vga_println,
};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    kernel::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    unsafe { memory::init(phys_mem_offset) };

    let mut frame_allocator =
        unsafe { allocator::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    memory::with_page_table_mut(|mapper| {
        allocator::init_heap(mapper, &mut frame_allocator).expect("heap initialization failed");
    });

    kernel::sys::rsdp::init();

    #[cfg(test)]
    test_main();

    vga_println!("Initing ATA...");
    match kernel::platform::ata::init() {
        Ok(ata) => {
            vga_println!("ATA primary status: {:?}", ata.primary.status());

            vga_println!(
                "Response incomplete: {}, fixed device: {}, removable media: {}, is ATA: {}",
                ata.primary.identity.response_incomplete(),
                ata.primary.identity.fixed_device(),
                ata.primary.identity.removable_media(),
                ata.primary.identity.is_ata_device(),
            );
            vga_println!(
                "{} cylinders, {} heads and {} sectors per track",
                ata.primary.identity.num_cylinders(),
                ata.primary.identity.num_heads(),
                ata.primary.identity.num_sectors_per_track()
            );
            vga_println!("Vendor ID: {:?}", ata.primary.identity.vendor_unique_id_1());
        }
        Err(e) => {
            vga_println!("ATA init failed: {:?}", e);
        }
    }

    vga_println!("Init done, starting executor, have fun!");
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(kernel::task::keyboard::print_keypresses()));
    executor.run();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga_println!("{}", info);
    kernel::platform::halt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test::panic_handler(info);
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    vga_println!("async number: {}", number);
}
