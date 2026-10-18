//! Sudoku parsing, validation and solving.
//!
//! The input files come in three layouts, all of which reduce to 81 digits
//! once whitespace is dropped:
//!
//! - `.sudo` and `.ldat`: one line of 81 digits
//! - `.dat`: nine rows of nine space separated digits, blocks split by
//!   extra spaces and blank lines
//!
//! `0` marks an empty cell. Some `.dat` files are deliberately malformed.

/// Row major, `0` for an empty cell.
pub type Grid = [[u8; 9]; 9];

/// Parse any of the input layouts into a grid.
pub fn parse(text: &str) -> Result<Grid, String> {
    todo!()
}

/// Check that the givens do not already contradict each other.
pub fn is_consistent(grid: &Grid) -> bool {
    todo!()
}

/// Fill `grid` in place. Returns `false` if there is no solution.
pub fn solve(grid: &mut Grid) -> bool {
    todo!()
}

/// Render in the `.dat` layout: blocks separated by two spaces and a blank
/// line.
pub fn format(grid: &Grid) -> String {
    todo!()
}
