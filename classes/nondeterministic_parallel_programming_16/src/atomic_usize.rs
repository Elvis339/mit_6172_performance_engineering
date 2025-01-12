use std::arch::asm;
use std::cell::UnsafeCell;

pub struct AtomicUsize {
    inner: UnsafeCell<usize>,
}

unsafe impl Send for AtomicUsize {}
unsafe impl Sync for AtomicUsize {}

impl AtomicUsize {
    pub const fn new(v: usize) -> Self {
        Self {
            inner: UnsafeCell::new(v),
        }
    }

    pub fn load(&self) -> usize {
        unsafe { *self.inner.get() }
    }

    #[cfg(target_arch = "x86_64")]
    pub fn store(&self, v: usize) {
        unsafe {
            asm!(
                "lock; xchg [{address}], {v}",
                address = in(reg) self.inner.get(),
                v = in(reg) v
            );
        }
    }

    #[cfg(target_arch = "x86_64")]
    pub fn fetch_add(&self, mut v: usize) -> usize {
        unsafe {
            asm!(
                "lock; xadd [{address}], {v}",
                address = in(reg) self.inner.get(),
                v = inout(reg) v,
            );
        }

        v
    }

    #[cfg(target_arch = "x86_64")]
    pub fn swap(&self, mut v: usize) -> usize {
        unsafe {
            asm!(
                "lock; xchg [{var}], {v}",
                var = in(reg) self.inner.get(),
                v = inout(reg) v
            );
        }

        v
    }

    #[cfg(target_arch = "x86_64")]
    pub fn compare_exchange(&self, current: usize, new: usize) -> Result<usize, usize> {
        let zf: u8; // the value of the zero flag
        let result: usize; // the value of the destination before the operation

        unsafe {
            asm!(
                "lock; cmpxchg [{address}], {new}", // the operation
                "mov {result}, rax", // store the accumulator value in `result`
                "sete {zf}", // store the ZF value in `zf`
                address = in(reg) self.inner.get(),
                new = in(reg) new,
                zf = out(reg_byte) zf,
                result = out(reg) result,
                in("rax") current, // place `current` in the accumulator to start
            );
        }

        if zf == 1 {
            Ok(result)
        } else {
            Err(result)
        }
    }

    #[cfg(target_arch = "aarch64")]
    pub fn store(&self, v: usize) {
        unsafe {
            asm!(
                "stlr {value}, [{address}]",
                address = in(reg) self.inner.get(),
                value = in(reg) v,
                options(nostack, preserves_flags)
            );
        }
    }

    // https://developer.arm.com/documentation/dui0801/l/A64-Data-Transfer-Instructions/LDADDA--LDADDAL--LDADD--LDADDL--LDADDAL--LDADD--LDADDL--A64-
    // ldadda -> Load-Add with Acquire-Release
    #[cfg(target_arch = "aarch64")]
    pub fn fetch_add(&self, mut v: usize) -> usize {
        let mut prev: usize;
        unsafe {
            asm!(
                "ldadda {value}, {prev}, [{address}]",
                address = in(reg) self.inner.get(),
                value = inout(reg) v,
                prev = out(reg) prev,
                options(nostack, preserves_flags)
            );
        }
        prev
    }

    #[cfg(target_arch = "aarch64")]
    pub fn swap(&self, mut v: usize) -> usize {
        let mut prev: usize;
        let mut success: u32; // Ensure `success` is a 32-bit register
        unsafe {
            loop {
                asm!(
                    "ldxr {prev}, [{address}]",      // Load the current value atomically
                    "stxr {success:w}, {v}, [{address}]", // Attempt to store the new value
                    address = in(reg) self.inner.get(),
                    prev = out(reg) prev,
                    v = in(reg) v,
                    success = out(reg) success,
                    options(nostack, preserves_flags)
                );

                if success == 0 {
                    break;
                }
            }
        }
        prev
    }

    #[cfg(target_arch = "aarch64")]
    pub fn compare_exchange(&self, current: usize, new: usize) -> Result<usize, usize> {
        const MAX_RETRIES: usize = 1000; // Limit to prevent infinite retries
        let mut retries = 0;
        let mut prev: usize; // The value loaded from the address
        let mut success: u32; // Status of the exclusive store operation (success = 0)

        unsafe {
            loop {
                // Atomically load the current value
                asm!(
                "ldaxr {prev}, [{address}]", // Load the current value at address
                address = in(reg) self.inner.get(),
                prev = out(reg) prev,
                options(nostack, preserves_flags)
                );

                // Check if the loaded value matches the expected value
                if prev != current {
                    return Err(prev); // Return the loaded value if comparison fails
                }

                // Atomically attempt to store the new value
                asm!(
                "stlxr {success:w}, {new}, [{address}]", // Attempt to store the new value
                address = in(reg) self.inner.get(),
                new = in(reg) new,
                success = out(reg) success,
                options(nostack, preserves_flags)
                );

                // If the store succeeded, return success
                if success == 0 {
                    return Ok(prev); // Successfully swapped, return the previous value
                }

                // Retry with backoff if store failed
                retries += 1;
                if retries > MAX_RETRIES {
                    panic!("compare_exchange failed after maximum retries");
                }

                // Yield or sleep to reduce contention
                std::thread::yield_now(); // Yield to other threads
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_store() {
        let counter = AtomicUsize::new(0);
        counter.store(10);
        assert_eq!(counter.load(), 10);
    }

    #[test]
    fn test_fetch_add() {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut threads = Vec::new();

        for _ in 0..4 {
            let ctr = Arc::clone(&counter);
            threads.push(std::thread::spawn(move || {
                for _ in 0..1_000_000 {
                    ctr.fetch_add(1);
                }
            }));
        }

        // Wait for all threads to finish
        for thread in threads {
            thread.join().unwrap();
        }

        assert_eq!(counter.load(), 4_000_000);
    }

    #[test]
    fn test_swap() {
        let atomic = Arc::new(AtomicUsize::new(42));
        assert_eq!(atomic.load(), 42);

        let atomic_cloned = Arc::clone(&atomic);
        let handle = std::thread::spawn(move || {
            atomic_cloned.swap(100);
        });

        let prev = atomic.swap(200);
        assert_eq!(atomic.load(), 200);

        handle.join().unwrap();
        assert_eq!(atomic.load(), 100);
    }
    #[test]
    fn test_compare_exchange() {
        let atomic = AtomicUsize::new(42);

        // Test a successful compare_exchange
        assert_eq!(atomic.compare_exchange(42, 100), Ok(42));
        assert_eq!(atomic.load(), 100);

        // Test a failed compare_exchange
        assert_eq!(atomic.compare_exchange(42, 200), Err(100));
        assert_eq!(atomic.load(), 100);
    }
}
