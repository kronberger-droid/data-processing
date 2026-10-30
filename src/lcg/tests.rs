use super::*;

// Reference sequences were computed independently in Python with
// `y = (a * y + c) % m`.

#[test]
fn ansi_c_first_values() {
    let mut rng = Lcg::ansi_c(1);
    let got: Vec<u64> = (0..5).map(|_| rng.rand()).collect();
    assert_eq!(got, [1103527590, 377401575, 662824084, 1147902781, 2035015474]);
}

#[test]
fn randu_first_values() {
    let mut rng = Lcg::randu(1);
    let got: Vec<u64> = (0..5).map(|_| rng.rand()).collect();
    assert_eq!(got, [65539, 393225, 1769499, 7077969, 26542323]);
}

#[test]
fn new_matches_the_presets() {
    let mut preset = Lcg::ansi_c(42);
    let mut manual = Lcg::new(1103515245, 12345, 1 << 31, 42);
    for _ in 0..100 {
        assert_eq!(preset.rand(), manual.rand());
    }
}

#[test]
fn same_seed_same_sequence() {
    let mut a = Lcg::ansi_c(7);
    let mut b = Lcg::ansi_c(7);
    for _ in 0..1000 {
        assert_eq!(a.rand(), b.rand());
    }
}

#[test]
fn full_period_small_lcg() {
    // a = 5, c = 1, m = 16 satisfies Hull-Dobell, so the period is m:
    // every residue appears exactly once before the sequence repeats.
    let mut rng = Lcg::new(5, 1, 16, 0);
    let got: Vec<u64> = (0..17).map(|_| rng.rand()).collect();
    assert_eq!(
        got,
        [1, 6, 15, 12, 13, 2, 11, 8, 9, 14, 7, 4, 5, 10, 3, 0, 1]
    );
}

#[test]
fn randu_triplets_lie_on_planes() {
    // y_(k+2) = 6 y_(k+1) - 9 y_k (mod 2^31) holds for RANDU, which is why
    // its triplets fall on 15 planes. Integer exact, so this pins down the
    // multiplier and modulus completely.
    let m: i64 = 1 << 31;
    let mut rng = Lcg::randu(1);
    let ys: Vec<i64> = (0..1000).map(|_| rng.rand() as i64).collect();
    for w in ys.windows(3) {
        assert_eq!(w[2], (6 * w[1] - 9 * w[0]).rem_euclid(m));
    }
}

#[test]
fn large_modulus_does_not_overflow() {
    // a * y needs ~128 bits here, so a u64 product would overflow (and panic
    // in debug builds).
    let (a, c, m) = (6364136223846793005, 1442695040888963407, u64::MAX);
    let mut rng = Lcg::new(a, c, m, m - 1);
    let got: Vec<u64> = (0..3).map(|_| rng.rand()).collect();
    assert_eq!(
        got,
        [13525302890751722017, 11103957605651056222, 2843980579475465557]
    );
}

#[test]
fn uni01_is_rand_over_m() {
    let mut rng = Lcg::new(5, 1, 16, 0);
    let got: Vec<f64> = (0..4).map(|_| rng.uni01()).collect();
    assert_eq!(got, [1.0 / 16.0, 6.0 / 16.0, 15.0 / 16.0, 12.0 / 16.0]);
}

#[test]
fn uni01_stays_in_half_open_unit_interval() {
    let mut rng = Lcg::new(5, 1, 16, 0);
    // Covers the full period, so 0 / 16 is hit and 1.0 must not be.
    for _ in 0..16 {
        let x = rng.uni01();
        assert!((0.0..1.0).contains(&x), "{x} out of [0, 1)");
    }
}

#[test]
fn uni01_mean_is_about_one_half() {
    let mut rng = Lcg::ansi_c(1);
    let n = 100_000;
    let mean = (0..n).map(|_| rng.uni01()).sum::<f64>() / n as f64;
    // Standard error is 1 / sqrt(12 n) ~ 0.0009, so 0.01 is a wide margin.
    assert!((mean - 0.5).abs() < 0.01, "mean {mean}");
}

#[test]
fn uni_hits_every_value_and_nothing_else() {
    let mut rng = Lcg::ansi_c(1);
    let imax = 9;
    let mut counts = [0usize; 10];
    for _ in 0..10_000 {
        let x = rng.uni(imax);
        assert!(x <= imax, "{x} > {imax}");
        counts[x as usize] += 1;
    }
    assert!(counts.iter().all(|&n| n > 0), "{counts:?}");
}

#[test]
fn uni_uses_high_bits() {
    // With m = 16 the low bit of the state alternates, so `rand() % 2` would
    // give 1, 0, 1, 0, ... Scaling uses the high bit instead: values below
    // 8 map to 0 and the rest to 1.
    let mut rng = Lcg::new(5, 1, 16, 0);
    let got: Vec<u64> = (0..8).map(|_| rng.uni(1)).collect();
    assert_eq!(got, [0, 0, 1, 1, 1, 0, 1, 1]);
}

// The expected messages keep these from passing on a `todo!()` panic.

#[test]
#[should_panic(expected = "imax")]
fn uni_rejects_imax_not_below_m() {
    Lcg::new(5, 1, 16, 0).uni(16);
}

#[test]
#[should_panic(expected = "modulus")]
fn new_rejects_zero_modulus() {
    Lcg::new(1, 0, 0, 0);
}

#[test]
#[should_panic(expected = "seed")]
fn new_rejects_seed_not_below_m() {
    Lcg::new(5, 1, 16, 16);
}
