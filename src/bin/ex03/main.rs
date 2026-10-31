//! Aufgabe 3, `exercises/Angaben03.pdf`: matrix products, sequential vs threads
//! vs processes.

use std::{
    fs,
    io::{self, BufRead, BufReader, BufWriter, Write},
    process::{Command, Stdio},
    thread,
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

/// Like `threaded`, with each chunk multiplied in a child process running
/// `worker`. Every child gets its own writer and reader thread: a pipe holds
/// less than one result, so a child nobody reads from stalls after one product.
fn forked(workers: usize, pairs: &[(Matrix, Matrix)]) -> Result<(Duration, f64)> {
    let chunk_size = pairs.len().div_ceil(workers);
    let exe = std::env::current_exe()?;

    let start = Instant::now();

    let results = thread::scope(|s| -> Result<Vec<Matrix>> {
        let mut children = Vec::new();
        let mut writers = Vec::new();
        for chunk in pairs.chunks(chunk_size) {
            let mut child = Command::new(&exe)
                .arg("worker")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;

            let stdin = child.stdin.take().unwrap();
            let handle = s.spawn(move || -> Result<()> {
                let mut writer = BufWriter::new(stdin);
                for (a, b) in chunk {
                    a.write_to(&mut writer)?;
                    b.write_to(&mut writer)?;
                }
                writer.flush()?;
                Ok(())
            });
            children.push((child, chunk.len()));
            writers.push(handle);
        }

        let readers: Vec<_> = children
            .into_iter()
            .map(|(mut child, n)| {
                s.spawn(move || -> Result<Vec<Matrix>> {
                    let mut reader = BufReader::new(child.stdout.take().unwrap());
                    let out = (0..n)
                        .map(|_| Matrix::read_from(&mut reader))
                        .collect::<Result<Vec<_>>>()?;
                    let status = child.wait()?;
                    ensure!(status.success(), "worker exited with {status}");
                    Ok(out)
                })
            })
            .collect();
        let mut results = Vec::with_capacity(pairs.len());
        for r in readers {
            results.extend(r.join().unwrap()?);
        }

        for w in writers {
            w.join().unwrap()?;
        }

        Ok(results)
    })?;

    let duration = start.elapsed();
    let checksum = results.iter().flat_map(|m| m.data()).sum::<f64>();
    Ok((duration, checksum))
}

/// Child side of `forked`: reads pairs from stdin until EOF and writes each
/// product to stdout.
fn worker() -> Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());

    loop {
        if reader.fill_buf()?.is_empty() {
            break;
        }
        let a = Matrix::read_from(&mut reader)?;
        let b = Matrix::read_from(&mut reader)?;

        a.matmul(&b)?.write_to(&mut writer)?;
    }

    writer.flush()?;
    Ok(())
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
    // `forked` starts this binary again with `worker` as its first argument.
    if std::env::args().nth(1).as_deref() == Some("worker") {
        return worker();
    }
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

        println!("\n sequential with size {n}:\n");

        record(
            n,
            "sequential",
            1,
            measure(expected, || sequential(&pairs))?,
        );

        println!("\n threaded with size {n}:\n");

        for workers in WORKERS {
            record(
                n,
                "threads",
                workers,
                measure(expected, || threaded(workers, &pairs))?,
            );
        }

        println!("\n forked with size {n}:\n");

        for workers in WORKERS {
            record(
                n,
                "processes",
                workers,
                measure(expected, || forked(workers, &pairs))?,
            );
        }
    }

    fs::create_dir_all("out")?;
    fs::write("out/ex03.csv", csv)?;
    println!("wrote out/ex03.csv");
    Ok(())
}
