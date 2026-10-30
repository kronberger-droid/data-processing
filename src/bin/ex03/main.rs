//! Aufgabe 3, `exercises/Angaben03.pdf`: matrix products, sequential vs threads.

use std::{
    fs, thread,
    time::{Duration, Instant},
};

use anyhow::{Result, ensure};
use data_processing::lcg::Lcg;

mod matrix;

use matrix::Matrix;

/// Multiplies every pair in turn. Returns the time and a checksum of the results.
fn sequential(pairs: &[(Matrix, Matrix)]) -> Result<(Duration, f64)> {
    let start = Instant::now();

    let results = pairs
        .iter()
        .map(|(a, b)| a.matmul(b))
        .collect::<Result<Vec<_>>>()?;

    let duration = start.elapsed();

    let checksum = results.iter().flat_map(|m| m.data()).sum::<f64>();

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

    let checksum = results.iter().flat_map(|m| m.data()).sum::<f64>();

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
    let mut record = |n: usize,
                      method: &str,
                      workers: usize,
                      (min, median): (Duration, Duration)| {
        println!("n={n:<4} {method:<10} w={workers:<2}  min {min:>9.2?}  median {median:>9.2?}");
        let ms = |d: Duration| d.as_secs_f64() * 1e3;
        csv += &format!("{n},{method},{workers},{:.4},{:.4}\n", ms(min), ms(median));
    };

    for n in SIZES {
        let pairs: Vec<(Matrix, Matrix)> = (0..PRODUCTS)
            .map(|_| {
                (
                    Matrix::random(n, n, &mut rng),
                    Matrix::random(n, n, &mut rng),
                )
            })
            .collect();
        let (_, expected) = sequential(&pairs)?;

        record(
            n,
            "sequential",
            1,
            measure(expected, || sequential(&pairs))?,
        );
        for workers in WORKERS {
            record(
                n,
                "threads",
                workers,
                measure(expected, || threaded(workers, &pairs))?,
            );
        }
    }

    fs::create_dir_all("out")?;
    fs::write("out/ex03.csv", csv)?;
    println!("wrote out/ex03.csv");
    Ok(())
}
