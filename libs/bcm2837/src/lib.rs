#![no_std]
#![feature(strict_provenance)]

pub use bcm2837_pac as pac;

mod peripherals;

// pub mod emmc;
pub mod pins;
// pub mod uart;
pub mod videocore;
