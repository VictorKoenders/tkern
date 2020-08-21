//! Glue for architectural different hardware.
//!
//! This module is designed to provide the exact same interface to the hardware that the kernel is running on.
//! It does this by having a seperate submodule for each architecture (x86_64 and aarch64 for now) that expose the exact same functions.

#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use self::x86_64::*;

#[cfg(target_arch = "aarch64")]
mod aarch64;

#[cfg(target_arch = "aarch64")]
pub use self::aarch64::*;
