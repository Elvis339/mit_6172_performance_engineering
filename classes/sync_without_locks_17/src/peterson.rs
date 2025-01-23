mod atomic;

use std::sync::{Arc, atomic::{AtomicBool, AtomicUsize, Ordering}};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn current_time() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let millis = now.as_millis() % 1000;
    let seconds = now.as_secs();
    let hours = (seconds / 3600) % 24;
    let minutes = (seconds / 60) % 60;
    let secs = seconds % 60;

    format!("{:02}:{:02}:{:02}:{:03}", hours, minutes, secs, millis)
}

fn run_thread(
    name: &str,
    self_wants: Arc<AtomicBool>,
    other_wants: Arc<AtomicBool>,
    turn: Arc<AtomicUsize>,
    priority: usize,
    shared_resource: Arc<AtomicUsize>,
) {
    for _ in 0..3 {
        println!("{} [{}] wants to enter the critical section.", current_time(), name);
        self_wants.store(true, Ordering::SeqCst);
        turn.store(priority, Ordering::SeqCst);

        while other_wants.load(Ordering::SeqCst) && turn.load(Ordering::SeqCst) == priority {
            println!("{} [{}] is waiting for the other thread...", current_time(), name);
            thread::sleep(Duration::from_millis(100));
        }

        // critical section
        println!("{} [{}] is in the critical section!", current_time(), name);
        let value = shared_resource.fetch_add(1, Ordering::SeqCst);
        println!("{} [{}] incremented the value to: {}", current_time(), name, value + 1);
        thread::sleep(Duration::from_millis(500));

        println!("{} [{}] is leaving the critical section.", current_time(), name);
        self_wants.store(false, Ordering::SeqCst);
    }
}

fn peterson_algorithm() {
    let a_wants = Arc::new(AtomicBool::new(false)); // Alice's flag
    let b_wants = Arc::new(AtomicBool::new(false)); // Bob's flag
    let turn = Arc::new(AtomicUsize::new(0)); // Turn variable (0 = Alice, 1 = Bob)
    let shared_resource = Arc::new(AtomicUsize::new(0)); // Critical section: shared counter

    // Alice's thread
    let alice = {
        let a_wants_clone = Arc::clone(&a_wants);
        let b_wants_clone = Arc::clone(&b_wants);
        let turn_clone = Arc::clone(&turn);
        let shared_clone = Arc::clone(&shared_resource);
        thread::spawn(move || {
            run_thread("Alice", a_wants_clone, b_wants_clone, turn_clone, 1, shared_clone);
        })
    };

    // Bob's thread
    let bob = {
        let a_wants_clone = Arc::clone(&a_wants);
        let b_wants_clone = Arc::clone(&b_wants);
        let turn_clone = Arc::clone(&turn);
        let shared_clone = Arc::clone(&shared_resource);
        thread::spawn(move || {
            run_thread("Bob", b_wants_clone, a_wants_clone, turn_clone, 0, shared_clone);
        })
    };

    alice.join().unwrap();
    bob.join().unwrap();

    println!(
        "{} [Main] Final value of the shared resource: {}",
        current_time(),
        shared_resource.load(Ordering::SeqCst)
    );
}

fn main() {
    peterson_algorithm();
}
