use std::ops::{Add, Mul};

/// CompressedSparseRow (CSR) speeds up large scientific computations by omitting zeroes
/// By Bentley's principle: "The fastest way to compute is to not compute at all"
///
/// Space-Time Tradeoff:
/// - Time: Faster computations by skipping zero multiplications
/// - Space: Uses 3 vectors to track non-zero values and their positions
///
/// Used in large sparse matrices where most elements are zero
/// Common in: Scientific computing, graph algorithms, ML feature matrices
pub struct CompressedSparseRow<T>
where
    T: Add<Output = T> + Mul<Output = T> + Default + Copy,
{
    values: Vec<T>,             // Non-zero values
    column_indices: Vec<usize>, // Column indices for non-zero elements
    row_pointers: Vec<usize>,   // Pointers to start of each row in values
    original_cols: usize,       // Store original matrix dimensions
}

impl<T> CompressedSparseRow<T>
where
    T: Add<Output = T> + Mul<Output = T> + Default + Copy + PartialEq,
{
    pub fn new(matrix: Vec<Vec<T>>) -> Self {
        let cols = matrix[0].len();
        let mut values = Vec::new();
        let mut column_indices = Vec::new();
        let mut row_pointers = vec![0];

        // Convert matrix to CSR format
        for row in matrix {
            for (col, &val) in row.iter().enumerate() {
                if val != T::default() {
                    values.push(val);
                    column_indices.push(col);
                }
            }
            row_pointers.push(values.len());
        }

        Self {
            values,
            column_indices,
            row_pointers,
            original_cols: cols,
        }
    }

    pub fn reconstruct(&self) -> Vec<Vec<T>> {
        let rows = self.row_pointers.len() - 1;
        let mut result = vec![vec![T::default(); self.original_cols]; rows];

        for row in 0..rows {
            let start = self.row_pointers[row];
            let end = self.row_pointers[row + 1];

            for idx in start..end {
                let col = self.column_indices[idx];
                result[row][col] = self.values[idx];
            }
        }

        result
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        let start = self.row_pointers[row];
        let end = self.row_pointers[row + 1];

        for idx in start..end {
            if self.column_indices[idx] == col {
                return self.values[idx];
            }
        }
        T::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparse_matrix() {
        let matrix = vec![
            vec![0, 0, 0, 0],
            vec![5, 8, 0, 0],
            vec![0, 0, 3, 0],
            vec![0, 6, 0, 0],
            vec![0, 2, 0, 1],
        ];

        let csr = CompressedSparseRow::new(matrix.clone());

        let reconstructed = csr.reconstruct();
        assert_eq!(reconstructed, matrix);

        assert_eq!(csr.get(1, 0), 5);
        assert_eq!(csr.get(1, 1), 8);
        assert_eq!(csr.get(2, 2), 3);
        assert_eq!(csr.get(3, 1), 6);
        assert_eq!(csr.get(4, 1), 2);
        assert_eq!(csr.get(4, 3), 1);
        assert_eq!(csr.get(0, 0), 0);
    }
}
