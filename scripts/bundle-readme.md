# data-processing

Exercises 1 to 4 of the data processing course, in Rust, with the plots in
Typst.

## Layout

| Path | Contents |
|---|---|
| `bin/` | The programs, built for x86_64 Linux and statically linked, so they need nothing installed. |
| `src/` | Their Rust sources. `src/bin/exNN` is exercise NN, `src/lcg.rs` the random number generator ex04 uses. |
| `Cargo.toml`, `Cargo.lock` | Build description for the sources. |
| `typst/` | The plots: each `.typ` source next to the `.pdf` compiled from it. |
| `data/sudokus/` | Input for `ex01`. |
| `out/` | Data written by `ex03` and `ex04`, which the plots read. |

## Exercises

| Program | Exercise | Reads | Writes | Plot |
|---|---|---|---|---|
| `ex01` | Sudoku solver | `data/sudokus/` | | |
| `ex02` | CPU load counter | `/proc/stat` | | |
| `ex03` | Matrix multiplication, sequential, threaded and multi-process | | `out/ex03.csv` | `typst/ex03.pdf` |
| `ex04` | Linear congruential generator and Monte Carlo integration | | `out/ansi_c.dat`, `out/randu.dat`, `out/sin_x.dat`, `out/sphere.dat` | `typst/ex04.pdf`, `typst/ex04_error.pdf`, `typst/ex04_ball.pdf` |

## Running

The programs look for `data/` and `out/` in the working directory, so start
them from this folder:

    ./bin/ex01

`ex03` and `ex04` overwrite the files in `out/`. The PDFs keep showing the
data they were compiled with until they are recompiled.

## Rebuilding

With a Rust toolchain (1.85 or newer) and Typst, from this folder:

    cargo build --release
    typst compile --root . typst/ex03.typ

Compiling a plot fetches the `lilaq` package on first use.
