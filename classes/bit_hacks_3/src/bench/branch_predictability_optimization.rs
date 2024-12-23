use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;

pub fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);

    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            result.push(a[i]);
            i += 1;
        } else {
            result.push(b[j]);
            j += 1;
        }
    }

    result.extend_from_slice(&a[i..]);
    result.extend_from_slice(&b[j..]);

    result
}

pub fn merge_branchless(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);

    while i < a.len() && j < b.len() {
        let cmp = (a[i] <= b[j]) as usize;
        let not_cmp = 1 - cmp;

        let min = b[j] ^ ((b[j] ^ a[i]) & (-(cmp as i32)));
        result.push(min);

        i += cmp;
        j += not_cmp;  // Changed from !cmp to not_cmp
    }

    result.extend_from_slice(&a[i..]);
    result.extend_from_slice(&b[j..]);

    result
}

fn generate_sorted_data(size: usize) -> Vec<i32> {
    let mut rng = StdRng::seed_from_u64(42);
    let mut data: Vec<i32> = (0..size).map(|_| rng.random_range(-500..1000)).collect();
    data.sort();
    data
}

fn bench_merges(c: &mut Criterion) {
    let mut group = c.benchmark_group("Merge Algorithms");

    for size in [10, 100, 1000, 10000].iter() {
        let a = generate_sorted_data(*size);
        let b = generate_sorted_data(*size);

        group.bench_with_input(
            BenchmarkId::new("Standard Merge", size),
            &(a.clone(), b.clone()),
            |bencher, (a, b)| bencher.iter(|| merge(black_box(a), black_box(b)))
        );

        group.bench_with_input(
            BenchmarkId::new("Branchless Merge", size),
            &(a.clone(), b.clone()),
            |bencher, (a, b)| bencher.iter(|| merge_branchless(black_box(a), black_box(b)))
        );
    }
    group.finish();
}

criterion_group!(benches, bench_merges);
criterion_main!(benches);