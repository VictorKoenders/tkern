//! Common slice utils

/// Common slice utils
pub trait SliceUtils<T> {
    /// Pop the first entry from the slice, returning the value. The slice will point at the next entry after this operation.
    fn pop_front(&mut self) -> Option<T>
    where
        T: Copy;
}

impl<'a, T> SliceUtils<T> for &'a [T] {
    fn pop_front(&mut self) -> Option<T>
    where
        T: Copy,
    {
        let (front, remaining) = self.split_first()?;
        *self = remaining;
        Some(*front)
    }
}
