use core::cmp::Ordering;
use core::convert::From;
use core::fmt;
use core::hash;
use core::mem::{self, MaybeUninit};
use core::num::NonZeroUsize;

/// `*const T` but non-zero and [covariant].
///
/// Unlike `*const T`, the pointer must always be non-null, even if the pointer
/// is never dereferenced. This is so that enums may use this forbidden value
/// as a discriminant -- `Option<ConstNonNull<T>>` has the same size as `*const T`.
/// However the pointer may still dangle if it isn't dereferenced.
///
/// Unlike `*const T`, `ConstNonNull<T>` was chosen to be covariant over `T`. This makes it
/// possible to use `ConstNonNull<T>` when building covariant types, but introduces the
/// risk of unsoundness if used in a type that shouldn't actually be covariant.
/// (The opposite choice was made for `*const T` even though technically the unsoundness
/// could only be caused by calling unsafe functions.)
///
/// Covariance is correct for most safe abstractions, such as `Box`, `Rc`, `Arc`, `Vec`,
/// and `LinkedList`. This is the case because they provide a public API that follows the
/// normal shared XOR mutable rules of Rust.
///
/// If your type cannot safely be covariant, you must ensure it contains some
/// additional field to provide invariance. Often this field will be a [`PhantomData`]
/// type like `PhantomData<Cell<T>>` or `PhantomData<&'a mut T>`.
///
/// Notice that `ConstNonNull<T>` has a `From` instance for `&T`. However, this does
/// not change the fact that mutating through a (pointer derived from a) shared
/// reference is undefined behavior unless the mutation happens inside an
/// [`UnsafeCell<T>`]. The same goes for creating a mutable reference from a shared
/// reference. When using this `From` instance without an `UnsafeCell<T>`,
/// it is your responsibility to ensure that `as_mut` is never called, and `as_ptr`
/// is never used for mutation.
///
/// [covariant]: https://doc.rust-lang.org/reference/subtyping.html
/// [`PhantomData`]: core::marker::PhantomData
/// [`UnsafeCell<T>`]: core::cell::UnsafeCell
#[repr(transparent)]
pub struct ConstNonNull<T: ?Sized> {
    pointer: *const T,
}

impl<T: Sized> ConstNonNull<T> {
    /// Creates a new `ConstNonNull` that is dangling, but well-aligned.
    ///
    /// This is useful for initializing types which lazily allocate, like
    /// `Vec::new` does.
    ///
    /// Note that the pointer value may potentially represent a valid pointer to
    /// a `T`, which means this must not be used as a "not yet initialized"
    /// sentinel value. Types that lazily allocate must track initialization by
    /// some other means.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr::ConstNonNull;
    ///
    /// let ptr = ConstNonNull::<u32>::dangling();
    /// // Important: don't try to access the value of `ptr` without
    /// // initializing it first! The pointer is not null but isn't valid either!
    /// ```
    #[must_use]
    #[inline]
    pub const fn dangling() -> Self {
        // SAFETY: mem::align_of() returns a non-zero usize which is then casted
        // to a *const T. Therefore, `ptr` is not null and the conditions for
        // calling new_unchecked() are respected.
        unsafe {
            let ptr = mem::align_of::<T>() as *const T;
            ConstNonNull::new_unchecked(ptr)
        }
    }

    /// Returns a shared references to the value. In contrast to [`as_ref`], this does not require
    /// that the value has to be initialized.
    ///
    /// [`as_ref`]: ConstNonNull::as_ref
    ///
    /// # Safety
    ///
    /// When calling this method, you have to ensure that all of the following is true:
    ///
    /// * The pointer must be properly aligned.
    ///
    /// * It must be "dereferenceable" in the sense defined in [the module documentation].
    ///
    /// * You must enforce Rust's aliasing rules, since the returned lifetime `'a` is
    ///   arbitrarily chosen and does not necessarily reflect the actual lifetime of the data.
    ///   In particular, while this reference exists, the memory the pointer points to must
    ///   not get mutated (except inside `UnsafeCell`).
    ///
    /// This applies even if the result of this method is unused!
    ///
    /// [the module documentation]: core::ptr#safety
    #[inline]
    #[must_use]
    pub const unsafe fn as_uninit_ref<'a>(&self) -> &'a MaybeUninit<T> {
        // SAFETY: the caller must guarantee that `self` meets all the
        // requirements for a reference.
        unsafe { &*self.cast().as_ptr() }
    }
    
    pub fn offset(self, offset: isize) -> ConstNonNull<T> {
        ConstNonNull::new(unsafe { self.pointer.offset(offset) }).unwrap()
    }
}

impl<T: ?Sized> ConstNonNull<T> {
    /// Creates a new `ConstNonNull`.
    ///
    /// # Safety
    ///
    /// `ptr` must be non-null.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr::ConstNonNull;
    ///
    /// let mut x = 0u32;
    /// let ptr = unsafe { ConstNonNull::new_unchecked(&mut x as *const _) };
    /// ```
    ///
    /// *Incorrect* usage of this function:
    ///
    /// ```rust,no_run
    /// use std::ptr::ConstNonNull;
    ///
    /// // NEVER DO THAT!!! This is undefined behavior. ⚠️
    /// let ptr = unsafe { ConstNonNull::<u32>::new_unchecked(std::ptr::null_mut()) };
    /// ```
    #[inline]
    pub const unsafe fn new_unchecked(ptr: *const T) -> Self {
        // SAFETY: the caller must guarantee that `ptr` is non-null.
        ConstNonNull { pointer: ptr as _ }
    }

    /// Creates a new `ConstNonNull` if `ptr` is non-null.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr::ConstNonNull;
    ///
    /// let mut x = 0u32;
    /// let ptr = ConstNonNull::<u32>::new(&mut x as *const _).expect("ptr is null!");
    ///
    /// if let Some(ptr) = ConstNonNull::<u32>::new(std::ptr::null_mut()) {
    ///     unreachable!();
    /// }
    /// ```
    #[inline]
    pub fn new(ptr: *const T) -> Option<Self> {
        if !ptr.is_null() {
            // SAFETY: The pointer is already checked and is not null
            Some(unsafe { Self::new_unchecked(ptr) })
        } else {
            None
        }
    }

    /// Gets the "address" portion of the pointer.
    ///
    /// For more details see the equivalent method on a raw pointer, `pointer::addr`.
    ///
    /// This API and its claimed semantics are part of the Strict Provenance experiment,
    /// see the [`ptr` module documentation][core::ptr].
    #[must_use]
    #[inline]
    pub fn addr(self) -> NonZeroUsize
    where
        T: Sized,
    {
        // SAFETY: The pointer is guaranteed by the type to be non-null,
        // meaning that the address will be non-zero.
        unsafe { NonZeroUsize::new_unchecked(self.pointer as usize) }
    }

    /// Acquires the underlying `*mut` pointer.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr::ConstNonNull;
    ///
    /// let mut x = 0u32;
    /// let ptr = ConstNonNull::new(&mut x).expect("ptr is null!");
    ///
    /// let x_value = unsafe { *ptr.as_ptr() };
    /// assert_eq!(x_value, 0);
    ///
    /// unsafe { *ptr.as_ptr() += 2; }
    /// let x_value = unsafe { *ptr.as_ptr() };
    /// assert_eq!(x_value, 2);
    /// ```
    #[must_use]
    #[inline]
    pub const fn as_ptr(self) -> *const T {
        self.pointer as *const T
    }

    /// Returns a shared reference to the value. If the value may be uninitialized, [`as_uninit_ref`]
    /// must be used instead.
    ///
    /// [`as_uninit_ref`]: ConstNonNull::as_uninit_ref
    ///
    /// # Safety
    ///
    /// When calling this method, you have to ensure that all of the following is true:
    ///
    /// * The pointer must be properly aligned.
    ///
    /// * It must be "dereferenceable" in the sense defined in [the module documentation].
    ///
    /// * The pointer must point to an initialized instance of `T`.
    ///
    /// * You must enforce Rust's aliasing rules, since the returned lifetime `'a` is
    ///   arbitrarily chosen and does not necessarily reflect the actual lifetime of the data.
    ///   In particular, while this reference exists, the memory the pointer points to must
    ///   not get mutated (except inside `UnsafeCell`).
    ///
    /// This applies even if the result of this method is unused!
    /// (The part about being initialized is not yet fully decided, but until
    /// it is, the only safe approach is to ensure that they are indeed initialized.)
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr::ConstNonNull;
    ///
    /// let mut x = 0u32;
    /// let ptr = ConstNonNull::new(&mut x as *const _).expect("ptr is null!");
    ///
    /// let ref_x = unsafe { ptr.as_ref() };
    /// println!("{ref_x}");
    /// ```
    ///
    /// [the module documentation]: core::ptr#safety
    #[must_use]
    #[inline]
    pub const unsafe fn as_ref<'a>(&self) -> &'a T {
        // SAFETY: the caller must guarantee that `self` meets all the
        // requirements for a reference.
        unsafe { &*self.as_ptr() }
    }

    /// Casts to a pointer of another type.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr::ConstNonNull;
    ///
    /// let mut x = 0u32;
    /// let ptr = ConstNonNull::new(&mut x as *const _).expect("null pointer");
    ///
    /// let casted_ptr = ptr.cast::<i8>();
    /// let raw_ptr: *const i8 = casted_ptr.as_ptr();
    /// ```
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub const fn cast<U>(self) -> ConstNonNull<U> {
        // SAFETY: `self` is a `ConstNonNull` pointer which is necessarily non-null
        unsafe { ConstNonNull::new_unchecked(self.as_ptr() as *const U) }
    }
}

impl<T> ConstNonNull<[T]> {
    /// Creates a non-null raw slice from a thin pointer and a length.
    ///
    /// The `len` argument is the number of **elements**, not the number of bytes.
    ///
    /// This function is safe, but dereferencing the return value is unsafe.
    /// See the documentation of [`core::slice::from_raw_parts`] for slice safety requirements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::ptr::ConstNonNull;
    ///
    /// // create a slice pointer when starting out with a pointer to the first element
    /// let mut x = [5, 6, 7];
    /// let nonnull_pointer = ConstNonNull::new(x.as_mut_ptr()).unwrap();
    /// let slice = ConstNonNull::slice_from_raw_parts(nonnull_pointer, 3);
    /// assert_eq!(unsafe { slice.as_ref()[2] }, 7);
    /// ```
    ///
    /// (Note that this example artificially demonstrates a use of this method,
    /// but `let slice = ConstNonNull::from(&x[..]);` would be a better way to write code like this.)
    #[must_use]
    #[inline]
    pub fn slice_from_raw_parts(data: ConstNonNull<T>, len: usize) -> Self {
        // SAFETY: `data` is a `ConstNonNull` pointer which is necessarily non-null
        unsafe { Self::new_unchecked(core::ptr::slice_from_raw_parts(data.as_ptr(), len)) }
    }
}

impl<T: ?Sized> Clone for ConstNonNull<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for ConstNonNull<T> {}

impl<T: ?Sized> fmt::Debug for ConstNonNull<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_ptr(), f)
    }
}

impl<T: ?Sized> fmt::Pointer for ConstNonNull<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_ptr(), f)
    }
}

impl<T: ?Sized> Eq for ConstNonNull<T> {}

impl<T: ?Sized> PartialEq for ConstNonNull<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}

impl<T: ?Sized> Ord for ConstNonNull<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ptr().cmp(&other.as_ptr())
    }
}

impl<T: ?Sized> PartialOrd for ConstNonNull<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ptr().partial_cmp(&other.as_ptr())
    }
}

impl<T: ?Sized> hash::Hash for ConstNonNull<T> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_ptr().hash(state)
    }
}

impl<T: ?Sized> From<&T> for ConstNonNull<T> {
    /// Converts a `&mut T` to a `ConstNonNull<T>`.
    ///
    /// This conversion is safe and infallible since references cannot be null.
    #[inline]
    fn from(reference: &T) -> Self {
        // SAFETY: A mutable reference cannot be null.
        ConstNonNull { pointer: reference as *const T }
    }
}
