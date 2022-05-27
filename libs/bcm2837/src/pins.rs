use paste::paste;

trait Pin {
    const ID: usize;
}

pub struct Unknown(());
pub struct Input(());
pub struct Output(());
pub struct Scl0(());
pub struct Sda0(());
pub struct Sa4(());
pub struct Sa5(());

macro_rules! impl_pins {
    ($(
        $idx:tt: {
            fsel: $fsel_parent:ident,
            fns: [
                $($fn:ident),*
            ],
        }
    )*) => {
        paste! {
            pub struct Pins {
                $(
                    pub [<p $idx>]: [<P $idx>] <Unknown>,
                )*
            }
            $(
                pub struct [<P $idx>] <TYPE>(TYPE);

                impl<T> [<P $idx>] <T> {
                    $(
                        pub fn $fn (self) -> [<P $idx>] < [<$fn:camel>] > {
                            crate::peripherals::GPIO::$fsel_parent(|sel|{
                                sel.modify(|_r, w| {
                                    w. [<fsel $idx>] ().$fn()
                                });
                            });
                            [<P $idx>](
                                [<$fn:camel>] (())
                            )
                        }
                    )*
                }
            )*
        }
    }
}

impl_pins! {
    0: {
        fsel: gpfsel0,
        fns: [ input, output, sda0, sa5 ],
    }
}
