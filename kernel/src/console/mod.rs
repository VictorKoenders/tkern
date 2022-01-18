pub mod uart;

pub use core::fmt::Write;

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
	uart::get().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => (
		$crate::console::_print(format_args!($($arg)*));
	)
}

#[macro_export]
macro_rules! println {
	() => (
		$crate::console::_print(format_args!("\n"));
	);
	($($arg:tt)*) => (
		$crate::console::_print(format_args_nl!($($arg)*));
	)
}
