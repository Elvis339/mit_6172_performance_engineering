use rayon::prelude::*;
use rand::random;
use std::sync::mpsc;
use std::time::Instant;

fn main() {
    const N: usize = 4096;

    let mut a = vec![vec![0.0; N]; N];
    let mut b = vec![vec![0.0; N]; N];

    for i in 0..N {
        for j in 0..N {
            a[i][j] = random::<f64>();
            b[i][j] = random::<f64>();
        }
    }

    let (tx, rx) = mpsc::channel::<(usize, usize, f64)>();

    // Spawn collector thread first
    let collector = std::thread::spawn(move || {
        let mut c = vec![vec![0.0; N]; N];
        while let Ok((i, j, val)) = rx.recv() {
            c[i][j] += val;
        }
        c
    });

    let start = Instant::now();

    // Multiple producer threads sending values
    (0..N).into_par_iter().for_each_with(tx, |tx, k| {
        for i in 0..N {
            for j in 0..N {
                let val = a[i][k] * b[k][j];
                tx.send((i, j, val)).unwrap();
            }
        }
    });

    let _ = collector.join().unwrap();
    let duration = start.elapsed();
    println!("{} sec", duration.as_secs_f64());
}
