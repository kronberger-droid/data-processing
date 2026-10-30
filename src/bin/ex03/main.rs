use std::ops::{Index, IndexMut};

use anyhow::{Result, ensure};

#[cfg(test)]
mod tests;

struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    fn row(&self, i: usize) -> &[f64] {
        &self.data[i * self.cols..(i + 1) * self.cols]
    }

    fn row_mut(&mut self, i: usize) -> &mut [f64] {
        &mut self.data[i * self.cols..(i + 1) * self.cols]
    }

    fn matmul(&self, other: &Matrix) -> Result<Matrix> {
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

fn main() -> Result<()> {
    Ok(())
}
