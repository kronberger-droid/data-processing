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
