#[macro_export]
macro_rules! numeric_enum {
    (
        $(#[$outer:meta])*
        pub enum $enum_name:ident : $type:ty{
            $(
                $(#[$inner:meta])*
                $variant_name:ident = $variant_value:tt
            ),*
        }
    ) => {numeric_enum!(__impl $(#[$outer])* [pub] enum $enum_name : $type {
        $(
            $(#[$inner])*
            $variant_name=$variant_value,
        )*
    });};
    (
        $(#[$outer:meta])*
        enum $enum_name:ident : $type:ty {
            $(
                $(#[$inner:meta])*
                $variant_name:ident = $variant_value:tt
            ),*
        }
    ) => {numeric_enum!(__impl $(#[$outer])* [] enum $enum_name : $type {
        $(
            $(#[$inner])*
            $variant_name=$variant_value,
        )*
    });};
    (
        $(#[$outer:meta])*
        pub enum $enum_name:ident : $type:ty{
            $(
                $(#[$inner:meta])*
                $variant_name:ident = $variant_value:tt,
            )*
        }
    ) => {numeric_enum!(__impl $(#[$outer])* [pub] enum $enum_name : $type {
        $(
            $(#[$inner])*
            $variant_name=$variant_value,
        )*
    });};
    (
        $(#[$outer:meta])*
        enum $enum_name:ident {
            $(
                $(#[$inner:meta])*
                $variant_name:ident = $variant_value:tt,
            )*
        }
    ) => {numeric_enum!(__impl $(#[$outer])* [] enum $enum_name : $type {
        $(
            $(#[$inner])*
            $variant_name=$variant_value,
        )*
    });};
    (
        __impl
        $(#[$outer:meta])*
        [$($vis:ident)?]
        enum $enum_name:ident : $type:ty {
            $(
                $(#[$inner:meta])*
                $variant_name:ident = $variant_value:tt,
            )*
        }
    ) => {
        $( #[$outer] )*
        $($vis)?
        enum $enum_name {
            $(
                $(#[$inner])*
                $variant_name,
            )*
            /// Unknown variant
            Unknown($type)
        }

        impl $enum_name {
            /// Create a new instance of this enum. Returns $enum_name::Unknown if a variant is not found.
            pub fn new(val: $type) -> Self {
                match val {
                    $($variant_value => Self::$variant_name,)*
                    x => Self::Unknown(x)
                }
            }

            /// Return the numeric value of the current variant
            pub fn into_numeric(self) -> $type {
                match self {
                    $(Self::$variant_name => $variant_value,)*
                    Self::Unknown(x) => x
                }
            }
        }
    };
}
