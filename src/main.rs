#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(kernel::test::runner))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]

use core::panic::PanicInfo;
use kernel::vga_println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga_println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test::panic_handler(info);
}
