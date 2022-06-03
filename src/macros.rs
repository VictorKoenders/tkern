#![allow(unused_macro_rules)]

macro_rules! println {
    () => {
        crate::output::print(format_args!("\n"));
    };
    ($e:expr) => {
        crate::output::print(format_args_nl!($e));
    };
    ($e:expr $(, $a: expr)*) => {
        crate::output::print(format_args_nl!($e $(, $a)*));
    }
}

macro_rules! print {
    ($e:expr) => {
        crate::output::print(format_args!($e));
    };
    ($e:expr $(, $a: expr)*) => {
        crate::output::print(format_args!($e $(, $a)*));
    }
}
