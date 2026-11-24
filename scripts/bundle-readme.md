# data-processing

Exercises of the data processing course, in Rust, with the plots in Typst.

- `bin/`: the programs, one per exercise, statically linked for x86_64 Linux.
- `src/`, `Cargo.toml`, `Cargo.lock`: their sources.
- `typst/`: the plots, each `.typ` source next to its `.pdf`.
- `data/`: input the programs read.
- `out/`: data the programs wrote, which the plots read.

The programs resolve `data/` and `out/` against the working directory, so
run them from this folder:

    ./bin/ex01
