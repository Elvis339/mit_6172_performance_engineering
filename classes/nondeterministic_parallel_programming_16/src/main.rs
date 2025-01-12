mod atomic_usize;

use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::sync::atomic::AtomicUsize;

fn main() {
    let lock = Arc::new(Mutex::new(0));
    let a = AtomicUsize::new(0);

    for i in 0..8 {
        let lock = Arc::clone(&lock);
        thread::spawn(move || {
            let start = Instant::now();
            let mut data = lock.lock().unwrap();
            *data += 1;
            // faster with drop
            // drop(data);
            let elapsed = start.elapsed();
            println!("Thread {} acquired lock after {:?}", i, elapsed)
        });
    }

    sleep(Duration::from_millis(500));
    println!("Done.")
}
