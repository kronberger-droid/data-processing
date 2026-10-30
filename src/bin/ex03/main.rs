use std::{
    ops::{Index, IndexMut},
    time::Instant,
};

use anyhow::{Result, ensure};
use data_processing::lcg::Lcg;

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

    fn random(rows: usize, cols: usize, rng: &mut Lcg) -> Self {
        let data = (0..rows * cols).map(|_| 2.0 * rng.uni01() - 1.0).collect();
        Self { rows, cols, data }
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
    let mut rng = Lcg::ansi_c(1);
    let (rows, cols) = (200, 200);
    let pairs: Vec<(Matrix, Matrix)> = (0..10)
        .map(|_| {
            (
                Matrix::random(rows, cols, &mut rng),
                Matrix::random(rows, cols, &mut rng),
            )
        })
        .collect();

    let start = Instant::now();

    let results = pairs
        .iter()
        .map(|(a, b)| a.matmul(b))
        .collect::<Result<Vec<_>>>()?;

    let duration = start.elapsed();

    let checksum = results.iter().flat_map(|m| &m.data).sum::<f64>();

    println!("took {duration:?} with checksum: {checksum}");

    Ok(())
}
