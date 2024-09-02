use alloc::boxed::Box;
use core::{
    ops::Deref,
    ptr,
    sync::atomic::{AtomicPtr, Ordering},
};

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
    pub const fn new(f: F) -> Self {
        Self {
            inner: AtomicPtr::new(ptr::null_mut()),
            f,
        }
    }

    pub fn force(this: &RacyLock<T, F>) -> &T {
        let mut ptr = this.inner.load(Ordering::Acquire);

        if ptr.is_null() {
            let val = (this.f)();
            ptr = Box::into_raw(Box::new(val));
            let exchange = this.inner.compare_exchange(
                ptr::null_mut(),
                ptr,
                Ordering::AcqRel,
                Ordering::Acquire,
            );
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
    fn test_racy_lock_copy() {
        // Lock a copy type and validate value.
        let lock = RacyLock::new(|| 42);
        assert_eq!(*lock, 42);
    }

    #[test]
    fn test_racy_lock_clone() {
        // Lock a no copy type.
        let lock = RacyLock::new(|| Vec::from([1, 2, 3]));

        // Use the value so that the compiler forces us to clone.
        let mut v = lock.clone();
        v.push(4);

        // Validate the value.
        assert_eq!(v, Vec::from([1, 2, 3, 4]));
    }

    #[test]
    fn test_racy_lock_static() {
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
    fn lazy_type_inference() {
        // Check that we can infer `T` from closure's type.
        let _ = RacyLock::new(|| ());
    }

    #[test]
    fn is_sync_send() {
        fn assert_traits<T: Send + Sync>() {}
        assert_traits::<RacyLock<Vec<i32>>>();
    }
}
