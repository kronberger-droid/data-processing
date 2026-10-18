//! Aufgabe 1: sudoku, GPS polygon and Laplace relaxation.
//!
//! Run from the repo root, the data paths are relative to it:
//!
//!     cargo run --bin aufgabe1 -- sudoku
//!     cargo run --bin aufgabe1 -- gps
//!     cargo run --bin aufgabe1 -- laplace [file]
//!
//! Results go to `out/aufgabe1/`, which is ignored.

use std::fs;
use std::path::Path;
use std::process::ExitCode;

use data_processing::{gps, laplace, sudoku};

const DATA: &str = "Aufgabe1_Ivan/daten";
const OUT: &str = "out/aufgabe1";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("sudoku") => run_sudoku(),
        Some("gps") => run_gps(),
        Some("laplace") => run_laplace(args.get(1).map(String::as_str)),
        _ => Err("usage: aufgabe1 <sudoku | gps | laplace [file]>".into()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn run_sudoku() -> Result<(), String> {
    let out = Path::new(OUT).join("sudoku");
    fs::create_dir_all(&out).map_err(|e| e.to_string())?;

    let mut files: Vec<_> = fs::read_dir(Path::new(DATA).join("sudokus"))
        .map_err(|e| e.to_string())?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();
    files.sort();

    for path in files {
        let name = path.file_name().unwrap().to_string_lossy();
        let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let mut grid = match sudoku::parse(&text) {
            Ok(grid) => grid,
            Err(e) => {
                println!("{name}: skipped, {e}");
                continue;
            }
        };
        if !sudoku::is_consistent(&grid) {
            println!("{name}: contradictory givens");
        } else if sudoku::solve(&mut grid) {
            let stem = path.file_stem().unwrap().to_string_lossy();
            fs::write(out.join(format!("{stem}.dat")), sudoku::format(&grid))
                .map_err(|e| e.to_string())?;
            println!("{name}: solved");
        } else {
            println!("{name}: no solution");
        }
    }
    Ok(())
}

fn run_gps() -> Result<(), String> {
    let text = fs::read_to_string(Path::new(DATA).join("Gpsies_austria.csv"))
        .map_err(|e| e.to_string())?;
    let points = gps::parse(&text)?;
    let c = gps::centroid(&points);
    println!("perimeter: {:.1} km", gps::perimeter(&points) / 1e3);
    println!("area:      {:.0} km²", gps::area(&points) / 1e6);
    println!("centroid:  {:.5}° N, {:.5}° E", c.lat, c.lon);
    Ok(())
}

fn run_laplace(file: Option<&str>) -> Result<(), String> {
    let path = Path::new(DATA)
        .join("laplace_daten")
        .join(file.unwrap_or("pl_ko30x30.dat"));
    let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut grid = laplace::parse(&text)?;
    let sweeps = laplace::relax(&mut grid, 1e-6, 100_000);
    println!("{}: converged after {sweeps} sweeps", path.display());
    Ok(())
}
