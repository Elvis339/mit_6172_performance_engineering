use crate::not_optimized_spinlock::SpinLock;
use crate::optimized_spinlock::OptimizedSpinLock;
use num_format::{Locale, ToFormattedString};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

// https://probablydance.com/2019/12/30/measuring-mutexes-spinlocks-and-how-bad-the-linux-scheduler-really-is/

mod atomic_usize;
mod not_optimized_spinlock;
mod optimized_spinlock;

const LOOPS: usize = 20_000;

fn benchmark_lock<F>(name: &str, lock_fn: F, threads: usize, loops: usize) -> Vec<u128>
where
    F: Fn() + Send + Sync + 'static,
{
    println!("Starting bench for {} loops {}", name, loops);
    let lock_fn = Arc::new(lock_fn);
    let (tx, rx) = channel();

    let handles: Vec<_> = (0..threads)
        .map(|_| {
            let tx = tx.clone();
            let lock_fn = Arc::clone(&lock_fn);
            thread::spawn(move || {
                for _ in 0..loops {
                    let start = Instant::now();
                    lock_fn();
                    let elapsed = start.elapsed().as_nanos();
                    tx.send(elapsed).unwrap();
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    drop(tx);

    rx.into_iter().collect()
}

fn fib(n: usize) -> usize {
    if n < 2 {
        return n;
    }

    return fib(n - 1) + fib(n - 2);
}

fn main() {
    let threads = thread::available_parallelism().unwrap().get() - 2;
    let rest = thread::available_parallelism().unwrap().get() - threads;

    // 6 threads will increment number which is protected with different kind of mutexes
    // 2 threads will calculate fib numbers simulating system which does something else beside just incrementing

    let mut handles = Vec::with_capacity(rest);
    for _ in 0..rest {
        handles.push(thread::spawn(move || {
            fib(50);
        }));
    }

    // Benchmark default `Mutex`
    let mutex = Arc::new(Mutex::new(0));
    let mutex_times = benchmark_lock(
        "Mutex",
        {
            let mutex = Arc::clone(&mutex);
            move || {
                let mut guard = mutex.lock().unwrap();
                *guard += 1;
            }
        },
        threads,
        LOOPS,
    );

    let spin_lock = Arc::new(OptimizedSpinLock::new(0));
    let spinlock_times = benchmark_lock(
        "OptimizedSpinLock",
        {
            let spin_lock = Arc::clone(&spin_lock);
            move || {
                spin_lock.lock();
                unsafe {
                    let prev = spin_lock.data();
                    *prev += 1;
                }
                spin_lock.unlock();
            }
        },
        threads,
        LOOPS,
    );

    let unoptimized_spinlock = Arc::new(SpinLock::new(0));
    let unoptimized_spinlock_times = benchmark_lock(
        "NotOptimizedSpinLock",
        {
            let not_optimized_spinlock = Arc::clone(&unoptimized_spinlock);
            move || {
                let mut guard = not_optimized_spinlock.lock().expect("failed to lock");
                *guard += 1;
            }
        },
        threads,
        LOOPS,
    );

    for handle in handles {
        handle.join().unwrap();
    }

    report_results("Mutex", &mutex_times);
    report_results("OptimizedSpinLock", &spinlock_times);
    report_results("NotOptimizedSpinLock", &unoptimized_spinlock_times);
}

fn report_results(name: &str, times: &[u128]) {
    let total: u128 = times.iter().sum();
    let average = total as f64 / times.len() as f64;

    let mut sorted_times = times.to_vec();
    sorted_times.sort_unstable();
    let min = sorted_times.first().unwrap_or(&0);
    let max = sorted_times.last().unwrap_or(&0);
    let median = if sorted_times.len() % 2 == 0 {
        let mid = sorted_times.len() / 2;
        (sorted_times[mid - 1] + sorted_times[mid]) as f64 / 2.0
    } else {
        sorted_times[sorted_times.len() / 2] as f64
    };

    println!("---");
    println!(
        "{}: {} samples, avg: {:.2} ns, total: {} ns, min: {} ns, max: {} ns, median: {:.2} ns",
        name,
        times.len().to_formatted_string(&Locale::en),
        average,
        total.to_formatted_string(&Locale::en),
        min.to_formatted_string(&Locale::en),
        max.to_formatted_string(&Locale::en),
        median
    );
}
