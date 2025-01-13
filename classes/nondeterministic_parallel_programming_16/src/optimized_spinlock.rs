use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::yield_now;

pub struct OptimizedSpinLock<T> {
    inner: UnsafeCell<T>,
    status: AtomicBool,
}

unsafe impl<T: Send> Send for OptimizedSpinLock<T> {}
unsafe impl<T: Send> Sync for OptimizedSpinLock<T> {}

impl<T> OptimizedSpinLock<T> {
    pub fn new(data: T) -> Self {
        Self {
            inner: UnsafeCell::new(data),
            status: AtomicBool::new(false),
        }
    }

    // try locking without blocking
    #[inline(always)]
    pub fn try_lock(&self) -> bool {
        !self.status.load(Ordering::Relaxed)
            && self
                .status
                .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
    }

    // acquires the lock, spinning and yielding as necessary.
    #[inline(always)]
    pub fn lock(&self) {
        let mut spin_count = 0;

        while !self.try_lock() {
            if spin_count < 16 {
                // hint to CPU
                std::hint::spin_loop();
                spin_count += 1;
            } else {
                // yield to allow other threads to progress
                // it's a hint to the OS
                yield_now();
                spin_count = 0;
            }
        }
    }

    #[inline(always)]
    pub fn unlock(&self) {
        self.status.store(false, Ordering::Release);
    }

    #[inline(always)]
    pub fn data(&self) -> *mut T {
        self.inner.get()
    }
}
