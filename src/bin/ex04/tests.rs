use std::f64::consts::PI;

use super::*;

/// Runs `write_triplets` into memory and parses every line into its numbers.
fn triplets(rng: &mut Lcg, n: usize) -> Vec<Vec<f64>> {
    let mut buf = Vec::new();
    write_triplets(rng, n, &mut buf).unwrap();
    String::from_utf8(buf)
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().expect("not a number"))
                .collect()
        })
        .collect()
}

#[test]
fn n_lines_of_three_values_in_the_unit_interval() {
    let got = triplets(&mut Lcg::ansi_c(1), 100);
    assert_eq!(got.len(), 100, "lines");
    for t in &got {
        assert_eq!(t.len(), 3, "values in {t:?}");
        for x in t {
            assert!((0.0..1.0).contains(x), "{x} out of [0, 1)");
        }
    }
}

#[test]
fn constant_function_gives_the_exact_integral() {
    // Every sample contributes exactly 3, so there is no MC noise at all.
    let got = integrate(|_| 3.0, 1.0, 4.0, 1, 1000, &mut Lcg::ansi_c(1));
    assert_eq!(got, 9.0);
}

#[test]
fn samples_are_points_inside_the_cube() {
    // With a != 0 this catches a mapping like `a + u * b`, which overshoots b.
    let (a, b, dim, n) = (1.0, 4.0, 3, 1000);
    let mut calls = 0;
    integrate(
        |x| {
            calls += 1;
            assert_eq!(x.len(), dim, "point dimension");
            for c in x {
                assert!((a..b).contains(c), "{c} outside [{a}, {b})");
            }
            0.0
        },
        a,
        b,
        dim,
        n,
        &mut Lcg::ansi_c(1),
    );
    assert_eq!(calls, n, "one call per sample");
}

#[test]
fn constant_function_in_two_dimensions_scales_with_the_area() {
    let got = integrate(|_| 3.0, 0.0, 2.0, 2, 1000, &mut Lcg::ansi_c(1));
    assert_eq!(got, 12.0);
}

/// Asserts `got` is within 5 standard errors of `exact`, for an integrand with
/// standard deviation `sigma` over a domain of volume `volume`.
fn assert_within_5_sigma(got: f64, exact: f64, volume: f64, sigma: f64, n: usize) {
    let tolerance = 5.0 * volume * sigma / (n as f64).sqrt();
    assert!(
        (got - exact).abs() < tolerance,
        "{got} is not within {tolerance} of {exact}"
    );
}

// The sigmas below are worked out by hand from Var(f) = E[f^2] - E[f]^2.

const SAMPLES: usize = 100_000;

#[test]
fn integral_of_sin_over_half_a_period() {
    let got = integrate(|x| x[0].sin(), 0.0, PI, 1, SAMPLES, &mut Lcg::ansi_c(1));
    // E[f] = 2 / pi and E[f^2] = 1 / 2.
    let sigma = (0.5 - 4.0 / (PI * PI)).sqrt();
    assert_within_5_sigma(got, 2.0, PI, sigma, SAMPLES);
}

#[test]
fn integral_of_x_over_an_interval() {
    let got = integrate(|x| x[0], 0.0, 2.0, 1, SAMPLES, &mut Lcg::ansi_c(1));
    // Uniform on [0, 2], so sigma = 2 / sqrt(12).
    assert_within_5_sigma(got, 2.0, 2.0, 2.0 / 12f64.sqrt(), SAMPLES);
}

#[test]
fn integral_of_a_product_needs_independent_coordinates() {
    // With the same value in both coordinates the mean would be E[x^2] = 4 / 3
    // and the result 16 / 3 instead of 4.
    let got = integrate(|x| x[0] * x[1], 0.0, 2.0, 2, SAMPLES, &mut Lcg::ansi_c(1));
    // E[f] = 1 and E[f^2] = (4 / 3)^2 = 16 / 9.
    let sigma = (16.0f64 / 9.0 - 1.0).sqrt();
    assert_within_5_sigma(got, 4.0, 4.0, sigma, SAMPLES);
}
