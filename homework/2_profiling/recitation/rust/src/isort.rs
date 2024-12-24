use rand::random;
use std::time::Instant;

const CACHE_LINE_SIZE: usize = 64;
// the size of u32 is always 4 bytes
const BLOCK_SIZE: usize = CACHE_LINE_SIZE / 4;

#[inline(always)]
fn isort(arr: &mut [u32]) {
    let n = arr.len();

    for i in 1..n {
        let mut j = i;

        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn isort_unroll(arr: &mut [u32]) {
    let len = arr.len();

    for i in 1..len {
        let val = arr[i];
        let mut j = i;

        unsafe {
            let ptr = arr.as_mut_ptr();

            'unroll: {
                while j >= 4 {
                    if *ptr.add(j - 4) <= val {
                        break 'unroll;
                    }

                    *ptr.add(j) = *ptr.add(j - 1);
                    *ptr.add(j - 3) = *ptr.add(j - 4);
                    *ptr.add(j - 1) = *ptr.add(j - 2);
                    *ptr.add(j - 2) = *ptr.add(j - 3);
                    j -= 4;
                }
            }

            // help branch prediction
            if j <= 0 {
                break
            }

            while j > 0 {
                if *ptr.add(j - 1) <= val {
                    break;
                }
                *ptr.add(j) = *ptr.add(j - 1);
                j -= 1;
            }

            *ptr.add(j) = val;
        }
    }
}
fn sort_block(_arr: &mut [u32]) {}

#[inline(always)]
fn isort_block(_arr: &mut [u32]) {}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let argc = args.len();

    if argc != 3 && argc != 4 {
        eprintln!("Error: wrong number of arguments");
        eprintln!("Usage: {} <size> <iterations> [block,unroll]", args[0]);
    }

    // data size
    let n = args
        .get(1)
        .map(|items| items.parse::<usize>().unwrap_or(1000))
        .unwrap_or(1000);

    // number of iterations
    let k = args
        .get(2)
        .map(|iterations| iterations.parse::<u64>().unwrap_or(10))
        .unwrap_or(10);

    let mut data: Vec<u32> = Vec::with_capacity(n);

    let value = args.get(3).map(String::as_str).unwrap_or("");
    let mut logged = false;
    for _ in 0..k {
        for _ in 0..n {
            data.push(random());
        }

        let start = Instant::now();
        match value {
            "block" => {
                if !logged {
                    println!("Using block version");
                    logged = true;
                }
                isort_block(&mut data);
            }
            "unroll" => {
                if !logged {
                    println!("Using unroll version");
                    logged = true;
                }
                isort_unroll(&mut data);
            }
            _ => {
                if !logged {
                    println!("Using un-optimized version");
                    logged = true;
                }
                isort(&mut data);
            }
        }
        let end = start.elapsed();
        println!("{} s", end.as_secs_f64());
    }
}
