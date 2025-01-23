use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

// rustc -C opt-level=3 --emit=asm src/atomic.rs
fn main() {
    // the purpose of this file is to output assembly for ARM processor
    // since x86-64 has `cmpxchg` instruction for compare and swap, let's see what ARM is doing
    // in output search for `cas`
    std::hint::black_box(());
    COUNTER.compare_and_swap(0, 5, Relaxed);
    std::hint::black_box(());
    let _ = COUNTER.load(Relaxed);
}