#![no_std]
#![warn(unsafe_op_in_unsafe_fn)]

pub mod const_non_null;
use core::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum HumanReadableSize {
    Bytes(f32),
    KiloBytes(f32),
    MegaBytes(f32),
    GigaBytes(f32),
    TeraBytes(f32),
}

impl fmt::Display for HumanReadableSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (tag, value) = match self {
            Self::Bytes(b) => ("b", b),
            Self::KiloBytes(b) => ("kb", b),
            Self::MegaBytes(b) => ("mb", b),
            Self::GigaBytes(b) => ("gb", b),
            Self::TeraBytes(b) => ("tb", b),
        };
        write!(f, "{:.02} {}", value, tag)
    }
}

impl HumanReadableSize {
    pub fn new(size: usize) -> Self {
        let mut result = HumanReadableSize::Bytes(size as f32);
        while result.value() > 1024. {
            if let Some(new_format) = result.next() {
                result = new_format;
            } else {
                break;
            }
        }
        result
    }

    pub fn value(&self) -> f32 {
        match self {
            HumanReadableSize::Bytes(v)
            | HumanReadableSize::KiloBytes(v)
            | HumanReadableSize::MegaBytes(v)
            | HumanReadableSize::GigaBytes(v)
            | HumanReadableSize::TeraBytes(v) => *v,
        }
    }

    fn next(self) -> Option<Self> {
        match self {
            Self::Bytes(v) => Some(Self::KiloBytes(v / 1024.)),
            Self::KiloBytes(v) => Some(Self::MegaBytes(v / 1024.)),
            Self::MegaBytes(v) => Some(Self::GigaBytes(v / 1024.)),
            Self::GigaBytes(v) => Some(Self::TeraBytes(v / 1024.)),
            Self::TeraBytes(_) => None,
        }
    }
}
