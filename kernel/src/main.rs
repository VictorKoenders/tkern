#![feature(format_args_nl, panic_info_message)]
#![cfg_attr(target_os = "tkern", no_main)]
#![no_std]

#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64/boot.rs"]
mod arch_boot;
mod bsp;
mod driver;
mod macros;

pub fn kernel_main() -> ! {
    println!("[0] Hello from Rust!");
    panic!("Stopping here");
}

#[cfg(not(target_os = "tkern"))]
fn main() {
    kernel_main();
}

mod sys {
    use core::ffi::c_void;
    use core::panic::PanicInfo;
    use core::ptr;

    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        if let Some(args) = info.message() {
            crate::println!("\nKernel panic: {}", args);
        } else {
            crate::println!("\nKernel panic!");
        }
        loop {
            cortex_a::asm::wfi();
        }
    }

    #[no_mangle]
    pub unsafe fn memcpy(dest: *mut c_void, src: *const c_void, size: isize) -> *mut c_void {
        {
            let dest: *mut u8 = dest.cast();
            let src: *const u8 = src.cast();
            for i in 0..size {
                ptr::write(dest.offset(i), ptr::read(src.offset(i)));
            }
        }

        dest
    }

    #[no_mangle]
    // TODO: Does memset only get called with a i8/u8 value?
    pub unsafe fn memset(dest: *mut c_void, val: i32, num: isize) -> *mut c_void {
        {
            let dest: *mut u8 = dest.cast();
            for i in 0..num {
                ptr::write(dest.offset(i), val as u8);
            }
        }

        dest
    }
}
