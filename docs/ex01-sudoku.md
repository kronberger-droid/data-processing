---
tags: [ex01, sudoku]
sheet: "[[exercises/Angaben01-02.pdf]]"
part: Beispiel 1, option 1
due: 2026-10-19
code: src/bin/ex01.rs
run: cargo run --release --bin ex01
---

# Ex01: Sudoku solver

The sheet's method is candidate elimination: every cell holds a list of possible digits, and two rules shrink the lists until nothing changes. That solves the "easy" puzzles. Where the rules get stuck, the solver guesses and backtracks, so it also solves the hard ones.

## Candidates as bitmasks

Each cell stores its candidate list as a `u16` where bit $d$ set means digit $d$ is still possible:

$$ "mask" = sum_(d in "candidates") 2^d $$

An empty cell starts as `ALL = 0b11_1111_1110` (bits 1 to 9), a given digit $d$ as $2^d$. That makes the list operations single instructions:

| Operation | Mask |
|---|---|
| strike $d$ | `m & !(1 << d)` |
| is $d$ possible | `m & (1 << d) != 0` |
| number of candidates | `m.count_ones()` |
| the digit, if only one is left | `m.trailing_zeros()` |
| digits possible anywhere in a unit | OR over its nine cells |

The whole board is `[u16; 81]`, 162 bytes, so `Sudoku` is `Copy` and a guess is a plain copy.

## Indexing

Cells are numbered $i = 0, dots, 80$ row major, so cell $i$ sits in row $floor(i \/ 9)$ and column $i mod 9$. The nine cells $k = 0, dots, 8$ of each unit:

$$ "row"(r): 9 r + k $$

$$ "column"(c): c + 9 k $$

$$ "box"(b): s_b + 9 floor(k \/ 3) + (k mod 3), quad s_b = 27 floor(b \/ 3) + 3 (b mod 3) $$

where $s_b$ is the top left cell of box $b$, boxes also numbered row major. The box containing cell $i$ is

$$ b(i) = 3 floor(i \/ 27) + floor((i mod 9) \/ 3) $$

The 9 rows, 9 columns and 9 boxes give 27 units. Each cell has 20 peers: 8 in its row, 8 in its column, and 4 more in its box.

## Rules

1. **Naked single.** A cell with one candidate left is fixed, and that digit is struck from all 20 peers. This is the sheet's "bleibt in einer Liste nur noch eine Zahl übrig".
2. **Hidden single.** If digit $d$ is possible in exactly one cell of a unit, that cell must be $d$, even if it still lists other candidates. This is the sheet's "Zahlen, die in jedem Teilbereich nur genau einmal vorkommen".

`solve` applies both rules over the whole board until a full pass changes nothing, then reports one of three outcomes:

- **Solved**: every cell has exactly one candidate.
- **Contradiction**: a cell has no candidates left, or some unit's OR is not `ALL`, meaning a digit has no place in it.
- **Stuck**: neither, the rules simply ran out.

## Backtracking

When `solve` is stuck, `search` picks the undecided cell with the fewest candidates, tries each of them on a copy of the board and recurses. A contradiction abandons that branch, and the first solved branch is returned.

Picking the cell with the fewest candidates keeps the tree narrow: a cell with 2 candidates gives a 50 % guess and at most two branches, where a cell with 9 would give nine. Each guess is followed by the full rule pass again, so most wrong guesses die within a step or two.

## Input

The parser takes 81 digits with whitespace ignored, `0` for an empty cell. That covers all three layouts in `data/sudokus/`: the grid of `.dat` files, and the single 81 digit line of `.ldat` and `.sudo` files. All 17 files there solve.
