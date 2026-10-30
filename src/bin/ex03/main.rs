//! Aufgabe 3, `exercises/Angaben03.pdf`: matrix products, sequential vs threads.

use std::{
    fs,
    ops::{Index, IndexMut},
    thread,
    time::{Duration, Instant},
};

use anyhow::{Result, ensure};
use data_processing::lcg::Lcg;

#[cfg(test)]
mod tests;

/// Dense matrix, row major in one flat `Vec`.
struct Matrix {
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
    fn random(rows: usize, cols: usize, rng: &mut Lcg) -> Self {
        let data = (0..rows * cols).map(|_| 2.0 * rng.uni01() - 1.0).collect();
        Self { rows, cols, data }
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

/// Multiplies every pair in turn. Returns the time and a checksum of the results.
fn sequential(pairs: &[(Matrix, Matrix)]) -> Result<(Duration, f64)> {
    let start = Instant::now();

    let results = pairs
        .iter()
        .map(|(a, b)| a.matmul(b))
        .collect::<Result<Vec<_>>>()?;

    let duration = start.elapsed();

    let checksum = results.iter().flat_map(|m| &m.data).sum::<f64>();

    Ok((duration, checksum))
}

/// Like `sequential`, with the pairs split into one chunk per thread.
/// Results are joined in input order, so the checksum matches.
fn threaded(workers: usize, pairs: &[(Matrix, Matrix)]) -> Result<(Duration, f64)> {
    let chunk_size = pairs.len().div_ceil(workers);

    let start = Instant::now();

    let results = thread::scope(|s| -> Result<Vec<Matrix>> {
        let handles: Vec<_> = pairs
            .chunks(chunk_size)
            .map(|chunk| {
                s.spawn(move || {
                    chunk
                        .iter()
                        .map(|(a, b)| a.matmul(b))
                        .collect::<Result<Vec<_>>>()
                })
            })
            .collect();
        let mut results = Vec::with_capacity(pairs.len());
        for handle in handles {
            results.extend(handle.join().unwrap()?);
        }
        Ok(results)
    })?;

    let duration = start.elapsed();

    let checksum = results.iter().flat_map(|m| &m.data).sum::<f64>();

    Ok((duration, checksum))
}

const PRODUCTS: usize = 10;
const SIZES: [usize; 2] = [200, 500];
const WORKERS: [usize; 6] = [1, 2, 4, 5, 8, 10];
/// Runs per configuration. Single runs vary too much to compare.
const REPS: usize = 20;

/// Min and median over `REPS` runs. Every run's checksum must equal `expected`.
fn measure(
    expected: f64,
    mut run: impl FnMut() -> Result<(Duration, f64)>,
) -> Result<(Duration, Duration)> {
    let mut times = Vec::with_capacity(REPS);
    for _ in 0..REPS {
        let (duration, checksum) = run()?;
        ensure!(
            checksum == expected,
            "checksum {checksum} differs from sequential {expected}"
        );
        times.push(duration);
    }
    times.sort();
    Ok((times[0], times[REPS / 2]))
}

fn main() -> Result<()> {
    let mut rng = Lcg::ansi_c(1);
    let mut csv = String::from("n,method,workers,min_ms,median_ms\n");
    let mut record = |n: usize, method: &str, workers: usize, (min, median): (Duration, Duration)| {
        println!("n={n:<4} {method:<10} w={workers:<2}  min {min:>9.2?}  median {median:>9.2?}");
        let ms = |d: Duration| d.as_secs_f64() * 1e3;
        csv += &format!("{n},{method},{workers},{:.4},{:.4}\n", ms(min), ms(median));
    };

    for n in SIZES {
        let pairs: Vec<(Matrix, Matrix)> = (0..PRODUCTS)
            .map(|_| (Matrix::random(n, n, &mut rng), Matrix::random(n, n, &mut rng)))
            .collect();
        let (_, expected) = sequential(&pairs)?;

        record(n, "sequential", 1, measure(expected, || sequential(&pairs))?);
        for workers in WORKERS {
            record(n, "threads", workers, measure(expected, || threaded(workers, &pairs))?);
        }
    }

    fs::create_dir_all("out")?;
    fs::write("out/ex03.csv", csv)?;
    println!("wrote out/ex03.csv");
    Ok(())
}
