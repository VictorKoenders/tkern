#![no_std]
#![warn(unsafe_op_in_unsafe_fn, rust_2018_idioms, clippy::pedantic)]

pub mod atomic_mutex;
pub mod const_non_null;

use core::{cell::UnsafeCell, fmt, ops};

// TODO: Replace this with `u16` so we don't have to do float calculations
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

/// A cell that is safe to read from but unsafe to write to. Useful for init-once static variables.
pub struct ReadCell<T>(UnsafeCell<T>);

impl<T> ReadCell<T> {
    /// Initialize a new readcell
    pub const fn new(t: T) -> Self {
        Self(UnsafeCell::new(t))
    }

    /// Get a reference to the value in this readcell
    pub fn get(&self) -> &T {
        // Safety: if the contract of `.set` is upheld, there should be no writes to this readcell while we have a read reference.
        unsafe { &*self.0.get() }
    }

    /// Write a value to this readcell.
    ///
    /// # Safety
    ///
    /// The caller must ensure that there are no outstanding references to `.get` for this cell.
    pub unsafe fn set(&self, value: T) {
        unsafe {
            *self.0.get() = value;
        }
    }

    /// Get a copy of the inner `T`
    pub fn copied(&self) -> T
    where
        T: Copy,
    {
        *self.get()
    }

    /// Get a clone of the inner `T`
    pub fn cloned(&self) -> T
    where
        T: Clone,
    {
        self.get().clone()
    }
}

impl<T> ops::Deref for ReadCell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

unsafe impl<T> Sync for ReadCell<T> {}
unsafe impl<T> Send for ReadCell<T> {}
