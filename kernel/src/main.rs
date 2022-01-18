#![feature(format_args_nl, panic_info_message)]
#![cfg_attr(target_os = "rusty_kernel", no_std)]
#![no_main]

#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64/boot.rs"]
mod arch_boot;
mod bsp;
mod console;

pub fn kernel_main() -> ! {
    println!("[0] Hello from Rust!");
    panic!("Stopping here");
}

#[cfg(target_os = "rusty_kernel")]
mod sys {
    use core::ffi::c_void;
    use core::panic::PanicInfo;

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
                core::ptr::write(dest.offset(i), core::ptr::read(src.offset(i)));
            }
        }
        dest
    }

    #[no_mangle]
    pub unsafe fn memset(ptr: *mut c_void, value: i32, num: isize) -> *mut c_void {
        {
            let ptr: *mut u8 = ptr.cast();
            let value = value as u8;
            for i in 0..num {
                core::ptr::write(ptr.offset(i), value);
            }
        }
        ptr
    }
}
