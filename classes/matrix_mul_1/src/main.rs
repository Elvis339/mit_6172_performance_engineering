use rayon::prelude::*;
use rand::random;
use std::time::Instant;

fn main() {
    const N: usize = 4096;

    let mut a = vec![vec![0.0; N]; N];
    let mut b = vec![vec![0.0; N]; N];
    let mut c = vec![vec![0.0; N]; N];

    for i in 0..N {
        for j in 0..N {
            a[i][j] = random::<f64>();
            b[i][j] = random::<f64>();
        }
    }

    let start = Instant::now();

    c.par_iter_mut().enumerate().for_each(|(k, row)| {
        for i in 0..N {
            for j in 0..N {
                row[j] += a[i][k] * b[k][j];
            }
        }
    });

    let duration = start.elapsed();
    println!("{}", duration.as_secs_f64());
}
