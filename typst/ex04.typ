// Graphical spectral test: triplets (x_k, x_(k+1), x_(k+2)) seen edge-on along
// the RANDU plane normal (9, -6, 1). RANDU collapses onto 15 lines, ANSI C
// stays a cloud. Data from `cargo run --release --bin ex04`, compile from the
// repo root:
//   typst compile --root . typst/ex04.typ
#import "@preview/lilaq:0.5.0" as lq

#set page(width: auto, height: auto, margin: 1em)
#set text(size: 10pt)

#let load(name) = csv("/out/" + name + ".dat", delimiter: " ").map(r => r.map(float))

#let inner(a, b) = a.zip(b).map(((p, q)) => p * q).sum()
#let unit(a) = {
  let l = calc.sqrt(inner(a, a))
  a.map(c => c / l)
}
// Screen axes: u along the normal, so each plane shows up as one vertical
// line, and v perpendicular to it.
#let u = unit((9, -6, 1))
#let v = unit((2, 3, 0))

#let panel(name, title) = {
  let pts = load(name)
  lq.diagram(
    width: 7cm,
    height: 6cm,
    title: title,
    xlabel: $bold(u) dot bold(p)$,
    ylabel: $bold(v) dot bold(p)$,
    xlim: (-0.6, 1.0),
    ylim: (-0.05, 1.45),
    // Explicit, since lilaq 0.5.0 labels a tick past the upper xlim.
    xaxis: (ticks: (-0.4, 0, 0.4, 0.8)),
    lq.scatter(pts.map(p => inner(p, u)), pts.map(p => inner(p, v)), size: 1pt, stroke: none),
  )
}

#grid(
  columns: 2,
  gutter: 3em,
  panel("ansi_c", [ANSI C]),
  panel("randu", [RANDU]),
)
