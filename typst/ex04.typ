// Graphical spectral test: triplets (x_k, x_(k+1), x_(k+2)) in the unit cube,
// seen from four directions. Only exactly edge-on along the RANDU plane normal
// (9, -6, 1) do RANDU's 15 planes show up. Data from
// `cargo run --release --bin ex04`, compile from the repo root:
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
#let cross(a, b) = (
  a.at(1) * b.at(2) - a.at(2) * b.at(1),
  a.at(2) * b.at(0) - a.at(0) * b.at(2),
  a.at(0) * b.at(1) - a.at(1) * b.at(0),
)

// u along the RANDU plane normal, v across the planes' extent.
#let u = unit((9, -6, 1))
#let v = unit((2, 3, 0))
// u turned by `angle` towards u x v, so the view tilts out of the planes.
#let tilt(angle) = u.zip(cross(u, v)).map(((p, q)) => calc.cos(angle) * p + calc.sin(angle) * q)

// Each view is a pair of orthonormal screen axes.
#let views = (
  (title: [along $x_(k+2)$], a: (1, 0, 0), b: (0, 1, 0)),
  (title: [oblique], a: unit((1, -1, 0)), b: unit((1, 1, -2))),
  (title: [edge-on, 5° off], a: tilt(5deg), b: v),
  (title: [edge-on], a: u, b: v),
)

// The 12 cube edges join corners that differ in one coordinate.
#let corners = range(8).map(i => (calc.rem(i, 2), calc.rem(calc.quo(i, 2), 2), calc.quo(i, 4)))
#let edges = {
  let out = ()
  for i in range(8) {
    for k in range(3) {
      let j = i + calc.pow(2, k)
      if calc.rem(calc.quo(i, calc.pow(2, k)), 2) == 0 { out.push((corners.at(i), corners.at(j))) }
    }
  }
  out
}

#let panel(pts, view, title: none, ylabel: none) = {
  let (a, b) = (view.a, view.b)
  // Square limits around the cube's centre keep the projection undistorted.
  let (ca, cb) = (inner((0.5, 0.5, 0.5), a), inner((0.5, 0.5, 0.5), b))
  lq.diagram(
    width: 4.2cm,
    height: 4.2cm,
    title: title,
    ylabel: ylabel,
    xlim: (ca - 0.9, ca + 0.9),
    ylim: (cb - 0.9, cb + 0.9),
    xaxis: (ticks: none),
    yaxis: (ticks: none),
    ..edges.map(((p, q)) => lq.plot(
      (inner(p, a), inner(q, a)),
      (inner(p, b), inner(q, b)),
      mark: none,
      color: gray,
      stroke: 0.5pt,
    )),
    // Explicit color, since the 12 edges have already advanced the style cycle.
    lq.scatter(
      pts.map(p => inner(p, a)),
      pts.map(p => inner(p, b)),
      size: 0.7pt,
      stroke: none,
      color: lq.color.map.petroff10.at(0),
    ),
  )
}

#let rows = (("ansi_c", [ANSI C]), ("randu", [RANDU]))

#grid(
  columns: views.len(),
  gutter: 0.8em,
  ..rows
    .enumerate()
    .map(((r, (name, label))) => {
      let pts = load(name)
      views
        .enumerate()
        .map(((c, view)) => panel(
          pts,
          view,
          title: if r == 0 { view.title },
          ylabel: if c == 0 { label },
        ))
    })
    .flatten(),
)

#block(width: 18cm)[
  #set par(justify: true)
  Each point is a triplet $bold(p) = (x_k, x_(k+1), x_(k+2))$ of consecutive `uni01` values,
  10 000 per generator, inside the unit cube (gray edges), projected orthographically onto two
  screen axes. _Along $x_(k+2)$_ plots $x_k$ against $x_(k+1)$. _Edge-on_ puts
  $bold(u) dot bold(p)$ on the horizontal axis, with $bold(u) = bold(n) slash |bold(n)|$ the unit
  normal of the RANDU planes, $bold(n) = (9, -6, 1)$, and $bold(v) dot bold(p)$ with
  $bold(v) = (2, 3, 0) slash sqrt(13)$ on the vertical one. Since
  $9 x_k - 6 x_(k+1) + x_(k+2) in ZZ$ for RANDU, $bold(u) dot bold(p)$ takes only 15 values spaced
  $1 slash sqrt(118) approx 0.092$ apart and every plane collapses to a vertical line. _5° off_
  turns $bold(u)$ by $5 degree$ towards $bold(u) times bold(v)$, and the lines already blur into
  bands. ANSI C looks the same from every direction.
]
