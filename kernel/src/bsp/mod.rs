cfg_if::cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        mod raspberrypi;
        pub use self::raspberrypi::*;
    } else {
        compile_error!(concat!("Unknown target arch ", env!("TARGET")));
    }
}
