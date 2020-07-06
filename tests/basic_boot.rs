#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test::runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kernel::vga_println;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    kernel::platform::halt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test::panic_handler(info)
}

#[test_case]
fn test_println() {
    vga_println!("test_println output");
}