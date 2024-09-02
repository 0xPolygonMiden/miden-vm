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
    use super::*;

    #[test]
    fn test_lazylock_copy() {
        let lock = RacyLock::new(|| 42);
        assert_eq!(*lock, 42);
    }

    #[test]
    fn test_lazylock_move() {
        let lock = RacyLock::new(|| vec![1, 2, 3]);
        assert_eq!(*lock, vec![1, 2, 3]);
    }
}
