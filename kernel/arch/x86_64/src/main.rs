#![no_std]
#![no_main]

extern crate alloc;

mod arch;
mod view;

use core::{mem::MaybeUninit, panic::PanicInfo};
use uefi::{
    prelude::entry,
    proto::console::{
        gop,
        text::{Key, ScanCode},
    },
    table::boot::SearchType,
    Handle,
};
use uefi_services::println;

#[entry]
fn efi_main(
    image: uefi::Handle,
    mut system_table: uefi::table::SystemTable<uefi::table::Boot>,
) -> uefi::Status {
    unsafe {
        uefi::alloc::init(system_table.boot_services());
    }
    system_table.stdout().clear().unwrap();
    uefi_services::init(&mut system_table).unwrap();
    try_render_square(&mut system_table);
    loop {
        if let Ok(Some(Key::Special(ScanCode::ESCAPE))) = system_table.stdin().read_key() {
            break;
        }
    }
    // view::run(&mut system_table);
    #[cfg(feature = "qemu")]
    {
        use qemu_exit::{QEMUExit, X86};
        X86::new(0xf4, 1).exit_success();
    }
    uefi::Status(0)
}

fn try_render_square(table: &mut uefi::table::SystemTable<uefi::table::Boot>) {
    let handles = table
        .boot_services()
        .locate_handle_buffer(SearchType::from_proto::<gop::GraphicsOutput>())
        .unwrap();
    println!("Found {} handles", handles.handles().len());
    for handle in handles.handles().iter().skip(1) {
        println!("  {handle:?}");
        match table
            .boot_services()
            .open_protocol_exclusive::<gop::GraphicsOutput>(*handle)
        {
            Ok(gop) => {
                let modes = gop.modes();
                println!("    {} modes", modes.len());
                for mode in modes {
                    let (width, height) = mode.info().resolution();
                    let pixel_format = mode.info().pixel_format();
                    let stride = mode.info().stride();
                    println!(
                        "        resolution: {width}x{height}, pixel format: {pixel_format:?}, stride: {stride}"
                    );
                }
            }
            Err(e) => {
                println!("    Could not load: {e:?}");
            }
        }
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
