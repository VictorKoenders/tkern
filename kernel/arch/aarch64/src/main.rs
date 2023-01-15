#![no_std]
#![no_main]

mod arch;

use core::arch::global_asm;
use core::fmt::Write;
use core::panic::PanicInfo;

global_asm!(include_str!("../boot.s"));

struct QemuWriter;
impl Write for QemuWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for char in s.chars() {
            unsafe { core::ptr::write_volatile(0x3F20_1000_usize as *mut u8, char as u8) }
        }
        Ok(())
    }
}

macro_rules! println {
    ($($arg:tt)*) => {{
        let _ = writeln!(&mut QemuWriter, $($arg)*);
    }}
}

#[no_mangle]
pub extern "C" fn _start_rust() -> ! {
    println!("Hello world");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
