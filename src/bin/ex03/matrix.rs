//! Dense `f64` matrix and its product.

use std::ops::{Index, IndexMut};

use anyhow::{Result, ensure};
use data_processing::lcg::Lcg;

#[cfg(test)]
mod tests;

/// Dense matrix, row major in one flat `Vec`.
pub(crate) struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    /// All entries 0.
    fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    /// Entries uniform in `[-1, 1)`.
    pub(crate) fn random(rows: usize, cols: usize, rng: &mut Lcg) -> Self {
        let data = (0..rows * cols).map(|_| 2.0 * rng.uni01() - 1.0).collect();
        Self { rows, cols, data }
    }

    /// All entries, row major.
    pub(crate) fn data(&self) -> &[f64] {
        &self.data
    }

    /// Row `i` as a slice.
    fn row(&self, i: usize) -> &[f64] {
        &self.data[i * self.cols..(i + 1) * self.cols]
    }

    /// Row `i` as a mutable slice.
    fn row_mut(&mut self, i: usize) -> &mut [f64] {
        &mut self.data[i * self.cols..(i + 1) * self.cols]
    }

    /// `self * other` in i-k-j order, so the inner loop walks rows.
    /// Errors if the inner dimensions differ.
    pub(crate) fn matmul(&self, other: &Matrix) -> Result<Matrix> {
        ensure!(self.cols == other.rows);
        let mut c = Matrix::zeros(self.rows, other.cols);

        for i in 0..c.rows {
            for k in 0..self.cols {
                let a_ik = self[(i, k)];
                for (c, b) in c.row_mut(i).iter_mut().zip(other.row(k)) {
                    *c += a_ik * b;
                }
            }
        }
        Ok(c)
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        debug_assert!(j < self.cols);
        &self.data[i * self.cols + j]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        debug_assert!(j < self.cols);
        &mut self.data[i * self.cols + j]
    }
}
