#[allow(unused_macros)]
macro_rules! info {
    ($e:expr $(, $a: expr)*) => {
        crate::output::info(|w| {
            let _ = core::fmt::Write::write_fmt(w, format_args_nl!($e $(, $a)*));
        });
    }
}

#[allow(unused_macros)]
macro_rules! warn {
    ($e:expr $(, $a: expr)*) => {
        crate::output::warn(|w| {
            let _ = core::fmt::Write::write_fmt(w, format_args_nl!($e $(, $a)*));
        });
    }
}
