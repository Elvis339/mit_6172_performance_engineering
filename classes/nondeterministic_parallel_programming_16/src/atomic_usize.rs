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

    // #[cfg(target_arch = "aarch64")]
    // pub fn store(&self, v: usize) {
    //     unsafe {
    //         asm!(
    //         // load the new value into a register
    //         "2:",
    //         // Store exclusive - attempts to store the value
    //         "stlr {v}, [{address}]",
    //         address = in(reg) self.inner.get(),
    //         v = in(reg) v,
    //         );
    //     }
    // }
    //
    // #[cfg(target_arch = "aarch64")]
    // pub fn fetch_add(&self, v: usize) -> usize {
    //     unsafe {
    //         let mut prev: usize;
    //         let mut status: u32;
    //         asm!(
    //         // Loop until the store is successful
    //         "2:",
    //         // Load exclusive
    //         "ldxr {prev}, [{address}]",
    //         // Add to get new value
    //         "add {tmp}, {prev}, {v}",
    //         // Store exclusive
    //         "stxr {status}, {tmp}, [{address}]",
    //         // Check if store was successful
    //         "cbnz {status}, 2b",
    //         address = in(reg) self.inner.get(),
    //         v = in(reg) v,
    //         status = out(reg) status,
    //         prev = out(reg) prev,
    //         tmp = out(reg) _,
    //         options(nostack)
    //         );
    //         prev
    //     }
    // }
    //
    // #[cfg(target_arch = "aarch64")]
    // pub fn swap(&self, v: usize) -> usize {
    //     unsafe {
    //         let mut prev: usize;
    //         let mut status: usize;
    //         asm!(
    //         "2:",
    //         // Load exclusive
    //         "ldxr {prev}, [{address}]",
    //         // Store exclusive - try to store new value
    //         "stxr {status}, {v}, [{address}]",
    //         // If store failed, try again
    //         "cbnz {status}, 2b",
    //         address = in(reg) self.inner.get(),
    //         v = in(reg) v,
    //         status = out(reg) status,
    //         prev = out(reg) prev,
    //         options(nostack)
    //         );
    //         prev
    //     }
    // }

    #[cfg(target_arch = "aarch64")]
    pub fn compare_exchange(&self, current: usize, new: usize) -> Result<usize, usize> {
        todo!("aarch64 not supported yet")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

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
        let counter = Arc::new(AtomicUsize::new(0));

        {
            let ctr = Arc::clone(&counter);
            std::thread::spawn(move || {
                ctr.swap(10);
            })
            .join()
            .unwrap();
        }

        counter.fetch_add(1);

        assert_eq!(counter.load(), 11);
    }

    #[test]
    fn test_compare_exchange() {
        let counter = AtomicUsize::new(0);
        counter.compare_exchange(0, 1).unwrap();

        assert_eq!(counter.load(), 1);
        assert_eq!(counter.compare_exchange(0, 1).unwrap_err(), 1);
    }
}