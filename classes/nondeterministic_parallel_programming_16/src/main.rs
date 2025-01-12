mod atomic_usize;

use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let lock = Arc::new(Mutex::new(0));

    // visual test to show how mutex works in Rust
    // i wanted to test mutex fairness
    // rust mutex relies on OS
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
