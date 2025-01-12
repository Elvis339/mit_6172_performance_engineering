use crate::atomic_usize::AtomicUsize;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

// Very inefficient Mutex implementations that keeps spinning in order to acquire lock
// wasting CPU resources

// Rust std::sync::Mutex is implement via system call https://man7.org/linux/man-pages/man3/pthread_mutex_lock.3p.html
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

    // there's no need to explicitly implement unlock because lock returns MutexGuard
    // and MutexGuard implements drop
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.mutex.inner.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.inner.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        if std::thread::panicking() {
            self.mutex.status.store(2);
        } else {
            self.mutex.status.store(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    use std::thread::sleep;
    use std::time::{Duration, Instant};

    #[test]
    fn test_mutex() {
        let mutex = Arc::new(Mutex::new(0_usize));
        let mut threads = Vec::new();

        for _ in 0..4 {
            let mutex_ref = mutex.clone();

            threads.push(std::thread::spawn(move || {
                for _ in 0..1_000_000 {
                    let mut counter = mutex_ref.lock().unwrap();
                    *counter += 1;
                }
            }));
        }

        // Wait for all threads to finish
        for thread in threads {
            thread.join().unwrap();
        }

        assert_eq!(*mutex.lock().unwrap(), 4_000_000);
    }

    // so slow...
    #[test]
    fn test_measure() {
        let lock = Arc::new(Mutex::new(0));

        for i in 0..8 {
            let lock = Arc::clone(&lock);
            thread::spawn(move || {
                let start = Instant::now();
                let mut data = lock.lock().unwrap();
                *data += 1;
                let elapsed = start.elapsed();
                println!("Thread {} acquired lock after {:?}", i, elapsed)
            });
        }

        sleep(Duration::from_millis(500));
        println!("Done.")
    }
}
