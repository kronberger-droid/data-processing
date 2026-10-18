//! Solving the Laplace equation on a grid by relaxation.
//!
//! Input files (`laplace_daten/*.dat`) are whitespace separated rows of
//! cells. A cell written `*v` is a fixed boundary value `v`, a plain number
//! is a free cell with that starting value.

#[derive(Debug, Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    /// Row major, `width * height` entries.
    pub values: Vec<f64>,
    /// `true` where the cell was written `*v`.
    pub fixed: Vec<bool>,
}

pub fn parse(text: &str) -> Result<Grid, String> {
    todo!()
}

/// Relax the free cells until the largest change in one sweep drops below
/// `tolerance` or `max_iterations` is reached. Returns the sweeps used.
pub fn relax(grid: &mut Grid, tolerance: f64, max_iterations: usize) -> usize {
    todo!()
}

/// Field `E = -grad(phi)` per cell, as `[ex, ey]`.
pub fn field(grid: &Grid) -> Vec<[f64; 2]> {
    todo!()
}
