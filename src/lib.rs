#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks, abi_x86_interrupt)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
pub mod driver;
pub mod platform;
pub mod test;

pub fn init() {
    use crate::platform::*;

    gdt::init();
    interrupts::init();
    pic::init();
    interrupts::enable();
}
