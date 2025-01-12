use std::cell::UnsafeCell;
use crate::atomic_usize::AtomicUsize;

pub struct Mutex<T> {
    inner: UnsafeCell<T>,
    status: AtomicUsize,
}

// MutexGuard is a type which is given to the locking thread to gain access to the underlying data
// It's useful because we can automatically unlock the mutex when this struct is dropped or poison
// the mutex if the thread panics. This pattern is called (RAII)
// Constructing the guard locks the mutex
// Droping the guard unlocks the mutex
pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

#[derive(Debug)]
pub enum MutexError {
    Poisoned,
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    pub const fn new(inner: T) -> Self {
        Self {
            inner: UnsafeCell::new(inner),
            status: AtomicUsize::new(0),
        }
    }

    pub fn lock(&self) -> Result<MutexGuard<T>, MutexError> {
        loop {
            match self.status.compare_exchange(0, 1) {
                Ok(_) => break, // locked mutex
                Err(2) => return Err(MutexError::Poisoned),
                Err(_) => continue, // mutex locked, try again
            }
        }

        Ok(MutexGuard { mutex: self })
    }
}
