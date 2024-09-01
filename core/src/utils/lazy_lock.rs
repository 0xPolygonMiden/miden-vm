use alloc::boxed::Box;
use core::{
    ops::Deref,
    ptr,
    sync::atomic::{AtomicPtr, Ordering},
};

pub struct LazyLock<T, F = fn() -> Box<T>>
where
    F: Fn() -> Box<T>,
{
    inner: AtomicPtr<T>,
    f: F,
}

impl<T, F> LazyLock<T, F>
where
    F: Fn() -> Box<T>,
{
    pub const fn new(f: F) -> Self {
        Self {
            inner: AtomicPtr::new(core::ptr::null_mut()),
            f,
        }
    }
    pub fn force(this: &LazyLock<T, F>) -> &T {
        let mut ptr = this.inner.load(Ordering::Acquire);

        if ptr.is_null() {
            //ptr = &(this.f)() as *const T as *mut T;
            let val = (this.f)();
            ptr = Box::into_raw(val);
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

impl<T, F> Deref for LazyLock<T, F>
where
    F: Fn() -> Box<T>,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        LazyLock::force(self)
    }
}

impl<T, F> Drop for LazyLock<T, F>
where
    F: Fn() -> Box<T>,
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
    fn test_lazylock_force() {
        let once = LazyLock::new(|| Box::new(42));
        let value = LazyLock::force(&once);
        assert_eq!(*value, 42);
    }
}
