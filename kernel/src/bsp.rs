cfg_if::cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        pub use sys_aarch64::*;
    } else {
        compile_error!(concat!("Unknown target arch ", env!("TARGET")));
    }
}
