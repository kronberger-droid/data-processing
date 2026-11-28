//! Aufgabe 5, `exercises/Angaben05.pdf`: throws of loaded dice from the
//! `wuerfel_server`, and the statistics that tell how each die is loaded.
//!
//! A throw is a `u8` from 1 to 6. Every statistic takes the throws or their
//! counts as plain data, so none of them knows about the socket.

use std::{
    io::{BufReader, Read, Write},
    net::TcpStream,
    time::Duration,
};

use anyhow::{Context, Result};

#[cfg(test)]
mod tests;

/// The dice the server offers, as its `help` reply lists them.
const IDS: [u32; 20] = [
    0, 1, 2, 3, 4, 5, 6, 10, 11, 12, 13, 20, 21, 22, 23, 70, 71, 80, 90, 100,
];

/// How often each face came up, the count of face `f` at index `f - 1`.
/// Panics on a throw that is not a face from 1 to 6.
fn face_counts(throws: &[u8]) -> [usize; 6] {
    let mut counts = [0; 6];
    for &throw in throws {
        assert!((1..=6).contains(&throw), "faces must be between 1 and 6");
        counts[throw as usize - 1] += 1;
    }

    counts
}

/// Shannon entropy in bits of the distribution given by `counts`. An outcome
/// that never occurred contributes nothing.
fn entropy(counts: &[usize]) -> f64 {
    let n: usize = counts.iter().sum();
    counts
        .iter()
        .filter_map(|c| match c {
            0 => None,
            c => {
                let p = *c as f64 / n as f64;
                Some(-p * p.log2())
            }
        })
        .sum::<f64>()
}

/// Mean and standard deviation of the number of successes in `n` trials that
/// each succeed with probability `p`.
fn binomial(n: usize, p: f64) -> (f64, f64) {
    (n as f64 * p, (n as f64 * p * (1.0 - p)).sqrt())
}

/// How many binomial standard deviations each face count lies from what a
/// fair die gives for the same number of throws.
fn deviations(counts: &[usize; 6]) -> [f64; 6] {
    let (mean, sigma) = binomial(counts.iter().sum(), 1.0 / 6.0);
    counts.map(|c| (c as f64 - mean) / sigma)
}

/// How often face `i + 1` was directly followed by face `j + 1`, at `[i][j]`.
/// The pairs overlap, so `n` throws hold `n - 1` of them.
fn pair_counts(throws: &[u8]) -> [[usize; 6]; 6] {
    let mut result = [[0; 6]; 6];
    for w in throws.windows(2) {
        result[w[0] as usize - 1][w[1] as usize - 1] += 1;
    }

    result
}

/// The pair counts to expect if every throw is independent of the one before
/// it, for a die with the face frequencies of `counts`.
fn expected_pairs(counts: &[usize; 6]) -> [[f64; 6]; 6] {
    let n: usize = counts.iter().sum();
    let p = counts.map(|c| c as f64 / n as f64);
    let pairs = (n - 1) as f64;

    let mut result = [[0.0; 6]; 6];
    for i in 0..6 {
        for j in 0..6 {
            result[i][j] = pairs * p[i] * p[j]
        }
    }
    result
}

/// How far the measured pair counts are from the expected ones, as the sum
/// of `(observed - expected)² / expected` over all cells.
fn chi_square(observed: &[[usize; 6]; 6], expected: &[[f64; 6]; 6]) -> f64 {
    observed
        .as_flattened()
        .iter()
        .zip(expected.as_flattened())
        .filter(|(_, e)| **e > 0.0)
        .map(|(&o, &e)| (o as f64 - e).powi(2) / e)
        .sum()
}

fn main() -> Result<()> {
    let n = 100_000;

    // The widths leave room for a die that shows a single face, which at
    // 100 000 throws is some 700 sigma off on that face.
    print!("{:>3} {:>8}", "id", "entropy");
    for face in 1..=6 {
        print!(" {:>8}", format!("dev{face}"));
    }
    println!(" {:>10}", "chi2");

    for id in IDS {
        let throws = fetch_throws(n, id).with_context(|| format!("die {id}"))?;
        let counts = face_counts(&throws);
        let bits = entropy(&counts);
        let sigmas = deviations(&counts);
        let chi2 = chi_square(&pair_counts(&throws), &expected_pairs(&counts));

        print!("{id:>3} {bits:>8.4}");
        for s in sigmas {
            print!(" {s:>+8.1}");
        }
        println!(" {chi2:>10.1}");
    }

    Ok(())
}

/// Asks the dice server for `n` throws of die `id` with `throw <n> <id>`.
/// Whatever else the reply holds besides faces is skipped, so it is the
/// count that ends the read and not the end of a line or of the connection.
fn fetch_throws(n: usize, id: u32) -> Result<Vec<u8>> {
    let mut stream = TcpStream::connect(("localhost", 56701))?;
    // Without it a server that accepts and then never answers hangs the read.
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;

    writeln!(stream, "throw {n} {id}")?;

    let throws: Vec<u8> = BufReader::new(stream)
        .bytes()
        .filter_map(|b| match b {
            Ok(d @ b'1'..=b'6') => Some(Ok(d - b'0')),
            Ok(_) => None,
            Err(e) => Some(Err(e)),
        })
        .take(n)
        .collect::<std::io::Result<_>>()?;

    anyhow::ensure!(
        throws.len() == n,
        "server sent {} of {n} throws",
        throws.len()
    );

    Ok(throws)
}
