mod x86_64;

pub mod pic;
pub use self::x86_64::*;

#[test_case]
fn test_breakpoint_exception() {
    self::interrupts::bkpt();
}
