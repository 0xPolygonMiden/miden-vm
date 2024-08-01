#[cfg(feature = "std")]
pub use parking_lot::RwLock;

#[cfg(not(feature = "std"))]
pub use self::no_std::RwLock;

#[cfg(not(feature = "std"))]
mod no_std {
    use core::sync::atomic::{AtomicUsize, Ordering};

    use lock_api::RawRwLock;

    pub type RwLock<T> = lock_api::RwLock<Spinlock, T>;

    pub struct Spinlock {
        state: AtomicUsize,
        writer_wake_counter: AtomicUsize,
    }
    impl Spinlock {
        const fn new() -> Self {
            Self {
                state: AtomicUsize::new(0),
                writer_wake_counter: AtomicUsize::new(0),
            }
        }
    }
    unsafe impl RawRwLock for Spinlock {
        const INIT: Spinlock = Spinlock::new();

        type GuardMarker = lock_api::GuardSend;

        fn lock_shared(&self) {
            let mut s = self.state.load(Ordering::Relaxed);
            loop {
                if s & 1 == 0 {
                    match self.state.compare_exchange_weak(
                        s,
                        s + 2,
                        Ordering::Acquire,
                        Ordering::Relaxed,
                    ) {
                        Ok(_) => return,
                        Err(e) => s = e,
                    }
                }
                if s & 1 == 1 {
                    loop {
                        let next = self.state.load(Ordering::Relaxed);
                        if s == next {
                            core::hint::spin_loop();
                        } else {
                            s = next;
                            break;
                        }
                    }
                }
            }
        }
        fn try_lock_shared(&self) -> bool {
            let s = self.state.load(Ordering::Relaxed);
            if s & 1 == 0 {
                self.state
                    .compare_exchange_weak(s, s + 2, Ordering::Acquire, Ordering::Relaxed)
                    .is_ok()
            } else {
                false
            }
        }

        unsafe fn unlock_shared(&self) {
            if self.state.fetch_sub(2, Ordering::Release) == 3 {
                self.writer_wake_counter.fetch_add(1, Ordering::Release);
            }
        }

        fn lock_exclusive(&self) {
            let mut s = self.state.load(Ordering::Relaxed);
            loop {
                if s <= 1 {
                    match self.state.compare_exchange(
                        s,
                        usize::MAX,
                        Ordering::Acquire,
                        Ordering::Relaxed,
                    ) {
                        Ok(_) => return,
                        Err(e) => {
                            s = e;
                            continue;
                        }
                    }
                }

                if s & 1 == 0 {
                    if let Err(e) =
                        self.state.compare_exchange(s, s + 1, Ordering::Relaxed, Ordering::Relaxed)
                    {
                        s = e;
                        continue;
                    }
                }

                let w = self.writer_wake_counter.load(Ordering::Acquire);
                s = self.state.load(Ordering::Relaxed);

                if s >= 2 {
                    while self.writer_wake_counter.load(Ordering::Acquire) == w {
                        core::hint::spin_loop();
                    }
                    s = self.state.load(Ordering::Relaxed);
                }
            }
        }

        fn try_lock_exclusive(&self) -> bool {
            let s = self.state.load(Ordering::Relaxed);
            if s <= 1 {
                self.state
                    .compare_exchange(s, usize::MAX, Ordering::Acquire, Ordering::Relaxed)
                    .is_ok()
            } else {
                false
            }
        }

        unsafe fn unlock_exclusive(&self) {
            self.state.store(0, Ordering::Release);
            self.writer_wake_counter.fetch_add(1, Ordering::Release);
        }
    }
}
