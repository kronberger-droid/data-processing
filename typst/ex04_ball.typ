// Volume of the d-dimensional unit ball relative to its bounding cube [-1, 1]^d,
// analytic and by Monte Carlo. Data from `cargo run --release --bin ex04`,
// compile from the repo root:
//   typst compile --root . typst/ex04_ball.typ
#import "@preview/lilaq:0.5.0" as lq

#set page(width: auto, height: auto, margin: 1em)
#set text(size: 10pt)

#let rows = csv("/out/sphere.dat", delimiter: " ", row-type: dictionary)
#let ds = rows.map(r => int(r.dim))
#let exact = rows.map(r => float(r.at("V_b/V_c")))
#let mc = rows.map(r => float(r.at("V_b_mc/V_c")))

// Samples per dimension, as in `main`.
#let n = 1000000

// The hit count is binomial with p = V_ball / V_cube, so the relative error of
// p is sqrt((1 - p) / (n p)).
#let rel-err = exact.zip(mc).map(((e, m)) => calc.abs(m - e) / e)
#let expected = exact.map(p => calc.sqrt((1 - p) / (n * p)))

#grid(
  columns: 2,
  gutter: 1.5em,
  lq.diagram(
    width: 7cm,
    height: 6cm,
    yscale: "log",
    xlabel: [dimension $d$],
    ylabel: [$V_"ball" slash V_"cube"$],
    legend: (position: bottom + left),
    lq.plot(ds, exact, mark: none, label: [analytic]),
    lq.plot(ds, mc, stroke: none, label: [MC, $N = 10^6$]),
  ),
  lq.diagram(
    width: 7cm,
    height: 6cm,
    yscale: "log",
    xlabel: [dimension $d$],
    ylabel: [relative error],
    legend: (position: top + left),
    // From d = 2: at d = 1 the ball fills the cube, both errors are 0 and
    // have no place on a log axis.
    lq.plot(ds.slice(1), rel-err.slice(1), stroke: none, label: [MC]),
    lq.plot(ds.slice(1), expected.slice(1), mark: none, color: gray, stroke: (dash: "dashed"), label: [$1 sigma = sqrt((1 - p) slash (N p))$]),
  ),
)
