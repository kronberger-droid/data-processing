//! Aufgabe 4, `exercises/Angaben04.pdf`: spectral test of the LCG and Monte
//! Carlo integration.

use std::{
    fs::{self, File},
    io::{BufWriter, Write},
};

use anyhow::Result;
use data_processing::lcg::Lcg;

#[cfg(test)]
mod tests;

/// Writes `n` triplets of consecutive `uni01` values, one `x y z` line each.
fn write_triplets(rng: &mut Lcg, n: usize, w: &mut impl Write) -> Result<()> {
    for _ in 0..n {
        let [x, y, z] = std::array::from_fn(|_| rng.uni01());
        writeln!(w, "{x} {y} {z}")?;
    }
    Ok(())
}

/// Monte Carlo estimate of the integral of `f` over the cube `[a, b]^dim` from
/// `n` uniform samples. `f` gets one sample point of length `dim`.
fn integrate(
    mut f: impl FnMut(&[f64]) -> f64,
    a: f64,
    b: f64,
    dim: usize,
    n: usize,
    rng: &mut Lcg,
) -> f64 {
    assert!(n > 0, "need at least one sample");
    let mut sum = 0.0;
    let mut x = vec![0.0; dim];
    for _ in 0..n {
        x.fill_with(|| a + (b - a) * rng.uni01());
        sum += f(&x);
    }
    (b - a).powi(dim as i32) * sum / n as f64
}

fn main() -> Result<()> {
    let seed = 1;
    let n = 10_000;

    fs::create_dir_all("out")?;
    for (name, mut rng) in [("ansi_c", Lcg::ansi_c(seed)), ("randu", Lcg::randu(seed))] {
        let mut w = BufWriter::new(File::create(format!("out/{name}.dat"))?);
        write_triplets(&mut rng, n, &mut w)?;
        w.flush()?;
    }

    Ok(())
}
