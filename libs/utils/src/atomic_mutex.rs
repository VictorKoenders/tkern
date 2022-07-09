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

    pub fn try_lock(&self) -> Option<Guard<'_, T>> {
        if self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(Guard(self))
        } else {
            None
        }
    }

    pub fn lock(&self) -> Guard<'_, T> {
        loop {
            if let Some(guard) = self.try_lock() {
                break guard;
            }
            core::hint::spin_loop();
        }
    }
}

// Safety: the implementation of `lock()` should ensure send and sync safety
unsafe impl<T> Send for AtomicMutex<T> {}
// Safety: the implementation of `lock()` should ensure send and sync safety
unsafe impl<T> Sync for AtomicMutex<T> {}

pub struct Guard<'a, T>(&'a AtomicMutex<T>);

impl<'a, T> Guard<'a, T> {
    #[must_use]
    pub fn copy(&self) -> T
    where
        T: Copy,
    {
        *self.deref()
    }
}

impl<'a, T> Deref for Guard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety: AtomicMutex::lock() should have ensured we only have 1 reference to this cell
        unsafe { &*self.0.data.get() }
    }
}

impl<'a, T> DerefMut for Guard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: AtomicMutex::lock() should have ensured we only have 1 reference to this cell
        unsafe { &mut *self.0.data.get() }
    }
}

impl<'a, T> Drop for Guard<'a, T> {
    fn drop(&mut self) {
        self.0.locked.store(false, Ordering::Release);
    }
}
