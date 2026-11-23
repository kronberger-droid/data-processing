#!/usr/bin/env nu
# Assemble a folder to rsync to the university server: Rust and Typst sources,
# static x86_64 binaries, the data they produce and the plots as PDF. Run it in
# the dev shell, from anywhere in the repo:
#   nu scripts/bundle.nu
#   rsync -av submit/ <host>:<dir>/
# The folder mirrors the repo root, because the binaries resolve
# `data/sudokus` and `out/` against the working directory. On the server:
#   cd <dir>; ./bin/ex01

const target = "x86_64-unknown-linux-musl"
const bins = [ex01 ex02 ex03 ex04]
# Binaries whose output in `out/` feeds the plots.
const producers = [ex03 ex04]
const sources = [Cargo.toml Cargo.lock src]

def main [
    --out (-o): path = "submit" # folder to create, replaced if it exists
    --no-run # reuse the data already in `out/` instead of rerunning ex03 and ex04
] {
    let out = $out | path expand
    cd ($env.FILE_PWD | path dirname)

    ^cargo build --release --target $target
    let built = [target $target release] | path join

    if not $no_run {
        for bin in $producers {
            print $"running ($bin)"
            ^($built | path join $bin)
        }
    }

    rm -rf $out
    for dir in [bin data pdf typst] { mkdir ($out | path join $dir) }
    for item in $sources { cp -r $item $out }
    # Only the sources: plots rendered next to them stay behind, `pdf/` has them.
    cp ...(glob typst/*.typ) ($out | path join typst)
    cp -r data/sudokus ($out | path join data)
    cp -r out $out
    for bin in $bins { cp ($built | path join $bin) ($out | path join bin) }

    for plot in (glob typst/*.typ) {
        let pdf = $out | path join pdf $"($plot | path parse | get stem).pdf"
        ^typst compile --root . $plot $pdf
    }

    print $"bundle in ($out)"
    ls ...(glob ($out | path join "{bin,pdf}" "*")) | select name size
}
