use alloc::boxed::Box;
use core::{
    ops::Deref,
    ptr,
    sync::atomic::{AtomicPtr, Ordering},
};

/// Thread-safe, non-blocking, "first one wins" flavor of `once_cell::sync::OnceCell`
/// with the same interface as `std::sync::LazyLock`.
///
/// The underlying implementation is based on `once_cell::sync::race::OnceBox` which relies on
/// `core::atomic::AtomicPtr` to ensure that the data race results in a single successful
/// write to the relevant pointer, namely the first write.
///
/// Performs lazy evaluation and can be used for statics.
pub struct RacyLock<T, F = fn() -> T>
where
    F: Fn() -> T,
{
    inner: AtomicPtr<T>,
    f: F,
}

impl<T, F> RacyLock<T, F>
where
    F: Fn() -> T,
{
    /// Creates a new lazy, racy value with the given initializing function.
    pub const fn new(f: F) -> Self {
        Self {
            inner: AtomicPtr::new(ptr::null_mut()),
            f,
        }
    }

    /// Forces the evaluation of the locked value and returns a reference to
    /// the result. This is equivalent to the `Deref` impl, but is explicit.
    ///
    /// This method will not block the calling thread if another initialization
    /// routine is currently running.
    pub fn force(this: &RacyLock<T, F>) -> &T {
        let mut ptr = this.inner.load(Ordering::Acquire);

        // Pointer is not yet set, attempt to set it ourselves.
        if ptr.is_null() {
            // Execute the initialization function and allocate.
            let val = (this.f)();
            ptr = Box::into_raw(Box::new(val));

            // Attempt atomic store.
            let exchange = this.inner.compare_exchange(
                ptr::null_mut(),
                ptr,
                Ordering::AcqRel,
                Ordering::Acquire,
            );

            // Pointer already set, load.
            if let Err(old) = exchange {
                drop(unsafe { Box::from_raw(ptr) });
                ptr = old;
            }
        }

        unsafe { &*ptr }
    }
}

impl<T, F> Deref for RacyLock<T, F>
where
    F: Fn() -> T,
{
    type Target = T;

    /// Dereferences the value.
    ///
    /// This method will not block the calling thread if another initialization
    /// routine is currently running.
    #[inline]
    fn deref(&self) -> &T {
        RacyLock::force(self)
    }
}

impl<T, F> Drop for RacyLock<T, F>
where
    F: Fn() -> T,
{
    fn drop(&mut self) {
        let ptr = *self.inner.get_mut();
        if !ptr.is_null() {
            drop(unsafe { Box::from_raw(ptr) });
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use super::*;

    #[test]
    fn test_deref_copy() {
        // Lock a copy type and validate value.
        let lock = RacyLock::new(|| 42);
        assert_eq!(*lock, 42);
    }

    #[test]
    fn test_deref_clone() {
        // Lock a no copy type.
        let lock = RacyLock::new(|| Vec::from([1, 2, 3]));

        // Use the value so that the compiler forces us to clone.
        let mut v = lock.clone();
        v.push(4);

        // Validate the value.
        assert_eq!(v, Vec::from([1, 2, 3, 4]));
    }

    #[test]
    fn test_deref_static() {
        // Create a static lock.
        static VEC: RacyLock<Vec<i32>> = RacyLock::new(|| Vec::from([1, 2, 3]));

        // Validate that the address of the value does not change.
        let addr = &*VEC as *const Vec<i32>;
        for _ in 0..5 {
            assert_eq!(*VEC, [1, 2, 3]);
            assert_eq!(addr, &(*VEC) as *const Vec<i32>)
        }
    }

    #[test]
    fn test_type_inference() {
        // Check that we can infer `T` from closure's type.
        let _ = RacyLock::new(|| ());
    }

    #[test]
    fn test_sync_send() {
        fn assert_traits<T: Send + Sync>() {}
        assert_traits::<RacyLock<Vec<i32>>>();
    }
}
