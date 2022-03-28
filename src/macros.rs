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

/// Prints an info, with a newline.
#[macro_export]
macro_rules! info {
    ($string:expr) => ({
        let timestamp = $crate::bsp::time().now().as_duration();
        let timestamp_subsec_us = timestamp.subsec_micros();

        $crate::macros::_print(format_args_nl!(
            concat!("[  {:>6}.{:03}] ", $string),
            timestamp.as_secs(),
            timestamp_subsec_us / 1_000,
        ));
    });
    ($format_string:expr, $($arg:tt)*) => ({
        let timestamp = $crate::bsp::time().now().as_duration();
        let timestamp_subsec_us = timestamp.subsec_micros();

        $crate::macros::_print(format_args_nl!(
            concat!("[  {:>6}.{:03}] ", $format_string),
            timestamp.as_secs(),
            timestamp_subsec_us / 1_000,
            $($arg)*
        ));
    })
}

/// Prints a warning, with a newline.
#[macro_export]
macro_rules! warn {
    ($string:expr) => ({
        let timestamp = $crate::bsp::time().now().as_duration();
        let timestamp_subsec_us = timestamp.subsec_micros();

        $crate::macros::_print(format_args_nl!(
            concat!("[W {:>6}.{:03}] ", $string),
            timestamp.as_secs(),
            timestamp_subsec_us / 1_000,
        ));
    });
    ($format_string:expr, $($arg:tt)*) => ({
        let timestamp = $crate::bsp::time().now().as_duration();
        let timestamp_subsec_us = timestamp.subsec_micros();

        $crate::macros::_print(format_args_nl!(
            concat!("[W {:>6}.{:03}] ", $format_string),
            timestamp.as_secs(),
            timestamp_subsec_us / 1_000,
            $($arg)*
        ));
    })
}
