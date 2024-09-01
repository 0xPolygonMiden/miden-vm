use core::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem::ManuallyDrop,
    ops::Deref,
    ptr,
    sync::atomic::{AtomicPtr, Ordering},
};
use std::boxed::Box;

pub struct LazyLock<T, F>
where
    F: Fn() -> T,
{
    inner: AtomicPtr<T>,
    f: F,
}

impl<T, F> LazyLock<T, F>
where
    F: Fn() -> T,
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
            ptr = &(this.f)() as *const T as *mut T;
            let exchange = this.inner.compare_exchange(
                ptr::null_mut(),
                ptr,
                Ordering::AcqRel,
                Ordering::Acquire,
            );
            if let Err(old) = exchange {
                ptr = old;
            }
        }

        unsafe { &*ptr }
    }

    fn get(&self) -> Option<&T> {
        let ptr = self.inner.load(Ordering::Acquire);
        if ptr.is_null() {
            None
        } else {
            unsafe { Some(&*ptr) }
        }
    }

    //pub fn into_inner(mut this: Self) -> Result<&'static T, F> {
    //    this.get().ok_or_else(|| this.f)
    //}

    //    pub fn get_or_init(&self, init: F) -> &T {
    //	let mut ptr = self.inner.load(core::sync::atomic::Ordering::Acquire);
    //	if ptr.is_null() {
    //	    let data = unsafe { &mut *self.data.get() };
    //	    ptr = match ptr.is_null() {
    //		true => {
    //		    let value = ManuallyDrop::new(unsafe { init() });
    //		    let ptr = &*value as *const T as *mut T;
    //		    core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
    //		    data.value = value;
    //		    ptr
    //		}
    //		false => ptr,
    //	    };
    //	    self.inner.store(ptr, core::sync::atomic::Ordering::Release);
    //	}
    //	unsafe { &*ptr }
    //    }
}

impl<T, F> Deref for LazyLock<T, F>
where
    F: Fn() -> T,
{
    type Target = T;

    /// Dereferences the value.
    ///
    /// This method will block the calling thread if another initialization
    /// routine is currently running.
    #[inline]
    fn deref(&self) -> &T {
        LazyLock::force(self)
    }
}

impl<T, F> Drop for LazyLock<T, F>
where
    F: Fn() -> T,
{
    fn drop(&mut self) {
        let ptr = *self.inner.get_mut();
        if !ptr.is_null() {
            drop(unsafe { Box::from_raw(ptr) })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lazylock() {
        let once = LazyLock::new(|| 42);
        let value = LazyLock::force(&once);
        assert_eq!(*value, 42);
    }
}
