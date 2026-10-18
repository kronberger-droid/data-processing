//! Aufgabe 7 (`exercises/Angaben07.pdf`): potential from the Laplace
//! equation by relaxation.
//!
//! Run from the repo root, the data paths are relative to it:
//!
//!     cargo run --bin aufgabe7 -- [file in data/laplace_daten]

use std::fs;
use std::path::Path;
use std::process::ExitCode;

use data_processing::laplace;

const DATA: &str = "data/laplace_daten";

fn main() -> ExitCode {
    let file = std::env::args().nth(1);
    match run(file.as_deref().unwrap_or("pl_ko30x30.dat")) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn run(file: &str) -> Result<(), String> {
    let path = Path::new(DATA).join(file);
    let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut grid = laplace::parse(&text)?;
    let sweeps = laplace::relax(&mut grid, 1e-6, 100_000);
    println!("{}: converged after {sweeps} sweeps", path.display());
    Ok(())
}
