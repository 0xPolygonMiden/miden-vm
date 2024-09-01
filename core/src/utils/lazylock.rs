use core::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem::ManuallyDrop,
    ptr,
    sync::atomic::{AtomicPtr, Ordering},
};

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
