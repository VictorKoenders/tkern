#![no_std]
#![no_main]

use core::panic::PanicInfo;
use kernel::{
    serial_print, serial_println,
    test::{exit_qemu_failed, exit_qemu_success},
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu_failed();
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu_success();
}
