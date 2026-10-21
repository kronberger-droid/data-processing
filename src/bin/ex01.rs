//! Aufgabe 1, `exercises/Angaben01-02.pdf` (Beispiel 1): sudoku solver.
//!
//! Candidate elimination as on the sheet, backtracking where it gets stuck.
use anyhow::{Context, Result, anyhow};
use std::{array, fmt, fs, path::Path, str::FromStr};

/// Candidate mask with every digit possible. Bit `d` stands for digit `d`.
const ALL: u16 = 0b11_1111_1110;

/// Indices of the nine cells of one row, column or box.
type Unit = [usize; 9];

/// 81 candidate masks, row major.
#[derive(Clone, Copy, Debug)]
struct Sudoku {
    grid: [u16; 81],
}

#[derive(Clone, Copy, Debug)]
enum Outcome {
    Solved,
    Stuck,
    Contradiction,
}

impl Sudoku {
    fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        fs::read_to_string(path)
            .with_context(|| format!("reading {}", path.display()))?
            .parse()
    }

    fn row(r: usize) -> Unit {
        array::from_fn(|k| r * 9 + k)
    }

    fn column(c: usize) -> Unit {
        array::from_fn(|k| c + k * 9)
    }

    /// Boxes are numbered 0..9 row major; `start` is the top left cell.
    fn rec(b: usize) -> Unit {
        let start = (b / 3) * 27 + (b % 3) * 3;
        array::from_fn(|k| start + (k / 3) * 9 + k % 3)
    }

    fn rec_of(i: usize) -> usize {
        (i / 27) * 3 + (i % 9) / 3
    }

    /// All 27 rows, columns and boxes.
    fn units() -> impl Iterator<Item = Unit> {
        (0..9).flat_map(|n| [Self::row(n), Self::column(n), Self::rec(n)])
    }

    /// The 20 cells sharing a unit with `i` (some twice).
    fn peers(i: usize) -> impl Iterator<Item = usize> {
        [
            Self::row(i / 9),
            Self::column(i % 9),
            Self::rec(Self::rec_of(i)),
        ]
        .into_iter()
        .flatten()
        .filter(move |&j| j != i)
    }

    /// Strike `d` from cell `i`. True if it was still a candidate.
    fn remove(&mut self, i: usize, d: u32) -> bool {
        let before = self.grid[i];
        self.grid[i] &= !(1 << d);
        self.grid[i] != before
    }

    /// Set cell `i` to `d` and strike `d` from its peers. True if anything changed.
    fn fix(&mut self, i: usize, d: u32) -> bool {
        let mut changed = self.grid[i] != 1 << d;
        self.grid[i] = 1 << d;
        for j in Self::peers(i) {
            changed |= self.remove(j, d);
        }
        changed
    }

    /// If `d` fits in exactly one cell of `unit`, fix it there.
    fn hidden_single(&mut self, unit: Unit, d: u32) -> bool {
        let mut places = unit.into_iter().filter(|&k| self.grid[k] & (1 << d) != 0);
        match (places.next(), places.next()) {
            (Some(k), None) => self.fix(k, d),
            _ => false,
        }
    }

    /// A cell without candidates, or a digit without a place in some unit.
    fn is_contradiction(&self) -> bool {
        self.grid.contains(&0)
            || Self::units().any(|u| u.iter().fold(0, |acc, &k| acc | self.grid[k]) != ALL)
    }

    fn is_solved(&self) -> bool {
        self.grid.iter().all(|m| m.count_ones() == 1)
    }

    /// Apply both rules until nothing changes.
    fn solve(&mut self) -> Outcome {
        loop {
            let mut changed = false;
            for i in 0..81 {
                let cell = self.grid[i];
                if cell.count_ones() == 1 {
                    changed |= self.fix(i, cell.trailing_zeros());
                }
            }
            for unit in Self::units() {
                for d in 1..=9 {
                    changed |= self.hidden_single(unit, d);
                }
            }

            if self.is_contradiction() {
                return Outcome::Contradiction;
            }
            if !changed {
                break;
            }
        }

        if self.is_solved() {
            Outcome::Solved
        } else {
            Outcome::Stuck
        }
    }

    /// Solve, guessing on the cell with the fewest candidates when stuck.
    /// Returns the first solution found.
    fn search(mut self) -> Option<Self> {
        match self.solve() {
            Outcome::Solved => Some(self),
            Outcome::Contradiction => None,
            Outcome::Stuck => {
                let i = (0..81)
                    .filter(|&i| self.grid[i].count_ones() > 1)
                    .min_by_key(|&i| self.grid[i].count_ones())?;

                (1..=9)
                    .filter(|&d| self.grid[i] & (1 << d) != 0)
                    .find_map(|d| {
                        let mut guess = self;
                        guess.fix(i, d);
                        guess.search()
                    })
            }
        }
    }
}

/// The `.dat` layout, `-` for undecided cells.
impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, &m) in self.grid.iter().enumerate() {
            if m.count_ones() == 1 {
                write!(f, "{}", m.trailing_zeros())?;
            } else {
                f.write_str("-")?;
            }
            let n = i + 1;
            f.write_str(match n {
                _ if n % 27 == 0 => "\n\n",
                _ if n % 9 == 0 => "\n",
                _ if n % 3 == 0 => "  ",
                _ => " ",
            })?;
        }
        Ok(())
    }
}

/// 81 digits, whitespace ignored, `0` for an empty cell.
impl FromStr for Sudoku {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let cells = s
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| {
                let d = c
                    .to_digit(10)
                    .with_context(|| format!("not a digit: {c:?}"))?;
                Ok(if d == 0 { ALL } else { 1 << d })
            })
            .collect::<Result<Vec<u16>>>()?;

        let grid = cells
            .try_into()
            .map_err(|v: Vec<u16>| anyhow!("expected 81 digits, found {}", v.len()))?;
        Ok(Self { grid })
    }
}

fn main() -> Result<()> {
    let mut paths = fs::read_dir("data/sudokus")?
        .map(|e| e.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()?;
    paths.sort();

    for path in &paths {
        match Sudoku::from_file(path) {
            Ok(sudoku) => match sudoku.search() {
                Some(solved) => println!("{}: solved\n\n{solved}", path.display()),
                None => println!("{}: no solution", path.display()),
            },
            Err(e) => println!("{}: {e:#}", path.display()),
        }
    }
    Ok(())
}
