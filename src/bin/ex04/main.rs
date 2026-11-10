//! Aufgabe 4, `exercises/Angaben04.pdf`: spectral test of the LCG and Monte
//! Carlo integration.

use std::{
    f64::consts::PI,
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

/// Absolute error of the estimate of the integral of `sin` over `[0, pi]`,
/// which is 2, from `n` samples, averaged over the seeds `1..=seeds`.
fn sin_integral_error(n: usize, seeds: u64) -> f64 {
    let total: f64 = (1..=seeds)
        .map(|seed| {
            let estimate = integrate(|x| x[0].sin(), 0.0, PI, 1, n, &mut Lcg::ansi_c(seed));
            (estimate - 2.0).abs()
        })
        .sum();
    total / seeds as f64
}

/// Exact volume of the `d`-dimensional unit ball.
fn ball_volume(d: usize) -> f64 {
    let (mut v, start) = if d % 2 == 0 { (1.0, 2) } else { (2.0, 3) };
    for k in (start..=d).step_by(2) {
        v *= 2.0 * PI / k as f64;
    }
    v
}

/// Monte Carlo estimate of `ball_volume(d)` from `n` samples.
fn ball_volume_mc(d: usize, n: usize, rng: &mut Lcg) -> f64 {
    integrate(
        |x| {
            if x.iter().map(|c| c * c).sum::<f64>() <= 1.0 {
                1.0
            } else {
                0.0
            }
        },
        -1.0,
        1.0,
        d,
        n,
        rng,
    )
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

    let mut w = BufWriter::new(File::create("out/sin_x.dat")?);
    writeln!(w, "n err")?;
    for i in 0..=7 {
        let n = 10usize.pow(i);
        writeln!(w, "{n} {}", sin_integral_error(n, 100))?;
    }
    w.flush()?;

    let mut w = BufWriter::new(File::create("out/sphere.dat")?);
    writeln!(w, "dim V_b/V_c V_b_mc/V_c")?;
    let mut rng = Lcg::ansi_c(seed);
    for d in 1..=12 {
        let cube = 2f64.powi(d as i32);
        writeln!(
            w,
            "{d} {} {}",
            ball_volume(d) / cube,
            ball_volume_mc(d, 1_000_000, &mut rng) / cube
        )?;
    }
    w.flush()?;

    Ok(())
}
