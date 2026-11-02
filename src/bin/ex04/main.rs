//! Aufgabe 4, `exercises/Angaben04.pdf`: spectral test of the LCG and Monte
//! Carlo integration.

use std::io::Write;

use anyhow::Result;
use data_processing::lcg::Lcg;

#[cfg(test)]
mod tests;

/// Writes `n` triplets of consecutive `uni01` values, one `x y z` line each.
fn write_triplets(rng: &mut Lcg, n: usize, w: &mut impl Write) -> Result<()> {
    todo!()
}

fn main() -> Result<()> {
    Ok(())
}
