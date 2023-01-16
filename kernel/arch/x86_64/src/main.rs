#![no_std]
#![no_main]

extern crate alloc;

mod arch;
// mod view;

use core::panic::PanicInfo;
use uefi::{
    prelude::entry,
    proto::console::{
        gop::{self, FrameBuffer, ModeInfo},
        text::{Key, ScanCode},
    },
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
    let handle = table
        .boot_services()
        .get_handle_for_protocol::<gop::GraphicsOutput>()
        .unwrap();
    let mut gop = table
        .boot_services()
        .open_protocol_exclusive::<gop::GraphicsOutput>(handle)
        .unwrap();
    let mode_info = gop.current_mode_info();
    let mut fb = gop.frame_buffer();

    let pixel = Color::WHITE;
    for offset_x in 0..8 {
        for offset_y in 0..4 {
            for x in 0..50 {
                for y in 0..50 {
                    let x = x + (offset_x * 50);
                    let y = y + (offset_y * 100) + if offset_x % 2 == 1 { 50 } else { 0 };
                    pixel.write(&mut fb, (x, y), &mode_info);
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const WHITE: Color = Color::new(255, 255, 255);
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub fn write(self, target: &mut FrameBuffer, (x, y): (u16, u16), format: &ModeInfo) {
        let offset = (x as usize + y as usize * format.stride()) * 4;
        match format.pixel_format() {
            gop::PixelFormat::Rgb => unsafe {
                target.write_value(offset, [self.r, self.g, self.b]);
            },
            gop::PixelFormat::Bgr => unsafe {
                target.write_value(offset, [self.b, self.g, self.r]);
            },
            gop::PixelFormat::Bitmask => unimplemented!(),
            gop::PixelFormat::BltOnly => unimplemented!(),
        }
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
