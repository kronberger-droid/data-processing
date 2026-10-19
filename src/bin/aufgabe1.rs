//! Aufgabe 1, `exercises/Angaben01-02.pdf` (Beispiel 1).
//!
//! Pick one of:
//!
//! 1. Sudoku (`data/sudokus/`): solve at least the easy ones.
//!    - every cell starts with the candidate list 1..=9
//!    - strike each given from the lists in its row, column and box
//!    - a digit that appears in only one list of a row, column or box is
//!      fixed there; a list with one entry left is fixed too
//!    - repeat until nothing changes
//!
//! 2. GPS plot (`data/Gpsies_austria.csv`): perimeter, area and centroid.
//!    - lines are `latitude longitude elevation index`, space separated
//!    - points run so the interior is always on the left
//!    - distances along great circles ("Orthodrome")
//!    - ignore the elevation
//!
//! 3. Curved mirror: mirror `f(x)` on `[a, b]` with derivative `f'(x)`.
//!    - send a bundle of straight rays at it
//!    - reflect each ray at its intersection with the mirror
//!    - plot the result
//!
//! 4. Meteorite entering the atmosphere: `a(t)`, `v(t)`, `h(t)`.
//!    - gravity decreases with height
//!    - drag `F_r = k v^2`, `k = c_w A rho(h) / 2`
//!    - barometric density `rho(h) = rho_0 exp(-M g h / (R T))`, with
//!      `M = 0.02896 kg/mol`, `R = 8.314 J/(K mol)`
//!    - ignore everything else (supersonic, heating, ablation, ...)
//!    - iron, `V = 1 cm^3`, `m = 7.874 g`
//!    - `v_0` = 15, 25 and 35 km/s, entry at `h = 150 km`
//!    - integrate by summing over small `dt`:
//!      `a = F / m`, `v = sum a dt`, `h = sum v dt`

fn main() {}
