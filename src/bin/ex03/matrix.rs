//! Dense `f64` matrix: products and a binary format for the worker pipes.

use std::{
    io::{Read, Write},
    ops::{Index, IndexMut},
};

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

    /// Writes `rows` and `cols` as `u64`, then the entries as `f64`, all little endian.
    pub(crate) fn write_to(&self, w: &mut impl Write) -> Result<()> {
        w.write_all(&(self.rows as u64).to_le_bytes())?;
        w.write_all(&(self.cols as u64).to_le_bytes())?;
        for x in &self.data {
            w.write_all(&x.to_le_bytes())?;
        }
        Ok(())
    }

    /// Reads one matrix in the `write_to` format and leaves the rest of `r` alone.
    /// Errors if `r` ends early.
    pub(crate) fn read_from(r: &mut impl Read) -> Result<Matrix> {
        let mut bytes = [0u8; 8];

        r.read_exact(&mut bytes)?;
        let rows = u64::from_le_bytes(bytes) as usize;

        r.read_exact(&mut bytes)?;
        let cols = u64::from_le_bytes(bytes) as usize;

        let mut data = Vec::with_capacity(rows * cols);
        for _ in 0..rows * cols {
            r.read_exact(&mut bytes)?;
            data.push(f64::from_le_bytes(bytes));
        }

        Ok(Matrix { rows, cols, data })
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
