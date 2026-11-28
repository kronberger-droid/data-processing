#!/usr/bin/env nu
# Assemble a folder to rsync to the university server: Rust and Typst sources,
# static x86_64 binaries, the data they produce, the plots as PDF and
# `bundle-readme.md` as its `README.md`. Run it in
# the dev shell, from anywhere in the repo:
#   nu scripts/bundle.nu
#   rsync -av submit/ <host>:<dir>/
# The folder mirrors the repo root, because the binaries resolve
# `data/sudokus` and `out/` against the working directory. On the server:
#   cd <dir>; ./bin/ex01

const target = "x86_64-unknown-linux-musl"
# Binaries whose output in `out/` feeds the plots.
const producers = [ex03 ex04]
const sources = [Cargo.toml Cargo.lock src]

def main [
    --out (-o): path = "submit" # folder to create, replaced if it exists
    --no-run # reuse the data already in `out/` instead of rerunning ex03 and ex04
] {
    let out = $out | path expand
    cd ($env.FILE_PWD | path dirname)
    # Cargo makes one binary per entry of `src/bin`, a file or a folder.
    let bins = ls src/bin | get name | path parse | get stem

    ^cargo build --release --target $target
    let built = [target $target release] | path join

    if not $no_run {
        for bin in $producers {
            print $"running ($bin)"
            ^($built | path join $bin)
        }
    }

    rm -rf $out
    for dir in [bin data typst] { mkdir ($out | path join $dir) }
    for item in $sources { cp -r $item $out }
    cp scripts/bundle-readme.md ($out | path join README.md)
    cp -r data/sudokus ($out | path join data)
    cp -r out $out
    for bin in $bins { cp ($built | path join $bin) ($out | path join bin) }

    # Each plot source travels with a PDF compiled from the data just bundled.
    for plot in (glob typst/*.typ) {
        cp $plot ($out | path join typst)
        let pdf = $out | path join typst $"($plot | path parse | get stem).pdf"
        ^typst compile --root . $plot $pdf
    }

    print $"bundle in ($out)"
    ls ...(glob ($out | path join "{bin,typst}" "*")) | select name size
}
