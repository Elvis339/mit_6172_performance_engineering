fn fib(n: usize) -> usize {
    if n < 2 {
        return n;
    }

    fib(n-1) + fib(n-2)
}

// rustc -C opt-level=3 --emit asm=output.s src/main.rs
fn main() {
    let n = 12;
    fib(n);
}
