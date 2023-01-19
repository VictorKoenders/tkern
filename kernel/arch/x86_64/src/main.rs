#![no_std]
#![no_main]

extern crate alloc;

mod arch;
mod render;
// mod view;

use core::panic::PanicInfo;
use uefi::{
    prelude::entry,
    proto::console::text::{Input, Key, ScanCode},
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
    let mut renderer =
        render::Renderer::new(locate_protocol(system_table.boot_services()).unwrap());

    renderer.block(50, 50, 300, 300, |b| {
        for i in 0..30 {
            b.draw_text("Hello world").at(i * 10, i * 10);
        }
    });

    let mut input = locate_protocol::<Input>(system_table.boot_services()).unwrap();
    loop {
        if let Ok(Some(Key::Special(ScanCode::ESCAPE))) = input.read_key() {
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

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn locate_protocol<P: uefi::proto::Protocol>(
    boot: &uefi::prelude::BootServices,
) -> uefi::Result<uefi::table::boot::ScopedProtocol<P>> {
    let mut handle = [core::mem::MaybeUninit::uninit()];
    let result = boot.locate_handle(
        uefi::table::boot::SearchType::from_proto::<P>(),
        Some(handle.as_mut_slice()),
    );
    match result {
        Ok(1) => {}
        Err(e) if e.status() == uefi::Status::BUFFER_TOO_SMALL => {}
        Err(e) => return Err(e),
        Ok(_) => return Err(uefi::Error::new(uefi::Status::NOT_FOUND, ())),
    };
    let handle = unsafe { handle[0].assume_init() };
    boot.open_protocol_exclusive(handle)
}
