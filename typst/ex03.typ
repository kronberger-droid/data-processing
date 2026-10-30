// Speedup of the threaded matrix products over the sequential baseline.
// Data from `cargo run --release --bin ex03`, compile from the repo root:
//   typst compile --root . typst/ex03.typ
#import "@preview/lilaq:0.5.0" as lq

#set page(width: auto, height: auto, margin: 1em)
#set text(size: 10pt)

#let rows = csv("/out/ex03.csv", row-type: dictionary)
#let sizes = rows.map(r => int(r.n)).dedup()
#let workers = rows.filter(r => r.method == "threads").map(r => int(r.workers)).dedup()

/// Median speedup T_seq / T_P per worker count for matrix size `n`.
#let speedup(n) = {
  let at-n = rows.filter(r => int(r.n) == n)
  let base = float(at-n.find(r => r.method == "sequential").median_ms)
  at-n.filter(r => r.method == "threads").map(r => base / float(r.median_ms))
}

#lq.diagram(
  width: 10cm,
  height: 7cm,
  xlabel: [workers $P$],
  ylabel: [speedup $S = T_"seq" slash T_P$],
  legend: (position: top + left),
  lq.plot(workers, workers, mark: none, stroke: (dash: "dashed"), label: [ideal $S = P$]),
  ..sizes.map(n => lq.plot(workers, speedup(n), label: [$n = #n$])),
)
