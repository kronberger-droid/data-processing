// Error of the Monte Carlo estimate of the integral of sin over [0, pi] against
// the number of samples. Data from `cargo run --release --bin ex04`, compile
// from the repo root:
//   typst compile --root . typst/ex04_error.typ
#import "@preview/lilaq:0.5.0" as lq

#set page(width: auto, height: auto, margin: 1em)
#set text(size: 10pt)

#let rows = csv("/out/sin_x.dat", delimiter: " ", row-type: dictionary)
#let ns = rows.map(r => float(r.n))
#let errs = rows.map(r => float(r.err))

// Expected |error| of a Gaussian estimate is sqrt(2 / pi) times its standard
// deviation, V sigma / sqrt(n). Nothing is fitted to the data.
#let sigma = calc.sqrt(0.5 - 4 / (calc.pi * calc.pi))
#let c = calc.sqrt(2 / calc.pi) * calc.pi * sigma

#lq.diagram(
  width: 10cm,
  height: 7cm,
  xscale: "log",
  yscale: "log",
  xlabel: [samples $N$],
  ylabel: [mean absolute error],
  legend: (position: top + right),
  lq.plot(ns, errs, stroke: none, label: [MC, $integral_0^pi sin x dif x$, 100 seeds]),
  lq.plot(ns, ns.map(n => c / calc.sqrt(n)), mark: none, color: gray, stroke: (dash: "dashed"), label: [expected, $#calc.round(c, digits: 2) slash sqrt(N)$]),
)
