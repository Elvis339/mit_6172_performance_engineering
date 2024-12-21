mod compressed_sparse_row;
use utils::time_it;

const SIZE: usize = 8192;

fn main() {
    let (matrix_a, matrix_b) = generate_matrices();

    let result_sparse = time_it!("sparsity", sparsity(&matrix_a, &matrix_b));
    let result_regular = time_it!("non sparsity", non_sparsity(&matrix_a, &matrix_b));

    assert_eq!(result_sparse, result_regular, "Results don't match!");
}

fn generate_matrices() -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    // Column matrix A (SIZE x 1)
    let mut a: Vec<Vec<usize>> = vec![vec![0; 1]; SIZE];
    for i in 0..SIZE {
        a[i][0] = if i % 3 == 0 { 0 } else { (i % 10) + 1 };
    }

    // Square matrix B (SIZE x SIZE)
    let mut b = vec![vec![0; SIZE]; SIZE];
    for i in 0..SIZE {
        for j in 0..SIZE {
            // Create a sparse matrix with ~60% zeros
            b[i][j] = if (i + j) % 3 == 0 {
                0
            } else {
                ((i + j) % 10) + 1
            };
        }
    }

    (a, b)
}

// The idea of sparsity is to avoid storing and computing on zeroes.
/// “The fastest way to compute is not to compute at all”
fn sparsity(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut c = vec![vec![0; 1]; SIZE];

    for i in 0..SIZE {
        for k in 0..SIZE {
            let temp_b = b[i][k];
            let temp_a = a[k][0];

            if temp_a == 0 || temp_b == 0 {
                continue;
            }
            c[i][0] += temp_b * temp_a
        }
    }

    c
}
fn non_sparsity(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut c = vec![vec![0; 1]; SIZE];

    for i in 0..SIZE {
        for k in 0..SIZE {
            c[i][0] += b[i][k] * a[k][0];
        }
    }

    c
}
