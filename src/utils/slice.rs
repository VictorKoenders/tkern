pub trait SliceUtils<T> {
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
