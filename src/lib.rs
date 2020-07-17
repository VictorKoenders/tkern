#![no_std]
#![cfg_attr(test, no_main)]
#![feature(
    custom_test_frameworks,
    abi_x86_interrupt,
    alloc_error_handler,
    wake_trait,
    trait_alias
)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

#[macro_use]
pub mod driver;
pub mod allocator;
pub mod platform;
pub mod sys;
pub mod task;
pub mod test;

pub fn init() {
    use crate::platform::*;

    gdt::init();
    interrupts::init();
    pic::init();
    interrupts::enable();
}
