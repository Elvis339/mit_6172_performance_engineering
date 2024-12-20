use rayon::prelude::*;
use rand::random;
use std::time::Instant;
use std::arch::aarch64::*;

// https://developer.arm.com/documentation/102159/0400/Overview
// https://developer.arm.com/architectures/instruction-sets/intrinsics/
#[inline(always)]
unsafe fn matrix_multiply_neon(a: &[Vec<f64>], b: &[Vec<f64>], c: &mut [Vec<f64>], n: usize) {
    c.par_iter_mut().enumerate().for_each(|(k, row)| {
        // Process 2 elements at a time using NEON
        for i in 0..n {
            let mut j = 0;
            while j + 2 <= n {
                // Load 2 elements from matrix B
                let b_ptr = b[k][j..].as_ptr();
                let b_vals = vld1q_f64(b_ptr);

                // Load and duplicate the scalar value from matrix A
                let a_val = vdupq_n_f64(a[i][k]);

                // Load current values from matrix C
                let c_ptr = &row[j] as *const f64;
                let c_vals = vld1q_f64(c_ptr);

                let result = vfmaq_f64(c_vals, a_val, b_vals);

                // Store the result back to matrix C
                let c_ptr_mut = &mut row[j] as *mut f64;
                vst1q_f64(c_ptr_mut, result);

                j += 2;
            }

            // Handle remaining elements
            while j < n {
                row[j] += a[i][k] * b[k][j];
                j += 1;
            }
        }
    });
}

#[cfg(target_arch = "aarch64")]
fn main() {
    const N: usize = 4096;

    let mut a = vec![vec![0.0; N]; N];
    let mut b = vec![vec![0.0; N]; N];
    let mut c = vec![vec![0.0; N]; N];

    // Initialize matrices with random values
    for i in 0..N {
        for j in 0..N {
            a[i][j] = random::<f64>();
            b[i][j] = random::<f64>();
            c[i][j] = 0.0;
        }
    }

    let start = Instant::now();

    unsafe {
        matrix_multiply_neon(&a, &b, &mut c, N);
    }

    let duration = start.elapsed();
    println!("Time taken: {} seconds", duration.as_secs_f64());
}

#[cfg(not(target_arch = "aarch64"))]
fn main() {
    println!("This code requires an ARM64 processor");
}