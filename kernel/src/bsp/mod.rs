#[cfg(target_arch = "aarch64")]
#[path = "raspberrypi.rs"]
mod inner;

#[cfg(any(target_os = "windows", target_os = "linux"))]
#[path = "dummy.rs"]
mod inner;

pub use self::inner::*;
