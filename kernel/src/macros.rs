pub use core::fmt::Write;

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    crate::bsp::uart().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => (
		$crate::macros::_print(format_args!($($arg)*));
	)
}

#[macro_export]
macro_rules! println {
	() => (
		$crate::macros::_print(format_args!("\n"));
	);
	($($arg:tt)*) => (
		$crate::macros::_print(format_args_nl!($($arg)*));
	)
}
