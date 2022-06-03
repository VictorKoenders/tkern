use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

pub struct AtomicMutex<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

impl<T> AtomicMutex<T> {
    pub const fn new(val: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(val),
        }
    }

    pub fn lock<'a>(&'a self) -> AtomicMutexGuard<'a, T> {
        while self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }
        AtomicMutexGuard(self)
    }
}

// Safety: the implementation of `lock()` should ensure send and sync safety
unsafe impl<T> Send for AtomicMutex<T> {}
// Safety: the implementation of `lock()` should ensure send and sync safety
unsafe impl<T> Sync for AtomicMutex<T> {}

pub struct AtomicMutexGuard<'a, T>(&'a AtomicMutex<T>);

impl<'a, T> AtomicMutexGuard<'a, T> {
    pub fn copy(&self) -> T
    where
        T: Copy,
    {
        *self.deref()
    }
}

impl<'a, T> Deref for AtomicMutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety: AtomicMutex::lock() should have ensured we only have 1 reference to this cell
        unsafe { &*self.0.data.get() }
    }
}

impl<'a, T> DerefMut for AtomicMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: AtomicMutex::lock() should have ensured we only have 1 reference to this cell
        unsafe { &mut *self.0.data.get() }
    }
}

impl<'a, T> Drop for AtomicMutexGuard<'a, T> {
    fn drop(&mut self) {
        self.0.locked.store(false, Ordering::Release);
    }
}
