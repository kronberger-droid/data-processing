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
