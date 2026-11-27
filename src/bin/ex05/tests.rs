use super::*;

fn assert_close(got: f64, want: f64) {
    assert!((got - want).abs() < 1e-12, "{got} is not {want}");
}

#[test]
fn faces_are_counted_at_their_own_index() {
    assert_eq!(face_counts(&[1, 1, 6, 3]), [2, 0, 1, 0, 0, 1]);
}

#[test]
fn no_throws_give_no_counts() {
    assert_eq!(face_counts(&[]), [0; 6]);
}

#[test]
#[should_panic(expected = "face")]
fn a_throw_above_six_is_rejected() {
    face_counts(&[3, 7]);
}

#[test]
#[should_panic(expected = "face")]
fn a_throw_of_zero_is_rejected() {
    // Catches indexing with `throw - 1`, which wraps or underflows here.
    face_counts(&[0]);
}

#[test]
fn a_fair_die_has_the_entropy_of_six_equal_outcomes() {
    assert_close(entropy(&[5; 6]), 6f64.log2());
}

#[test]
fn a_die_that_always_shows_one_face_has_no_entropy() {
    // The five empty faces must count as 0, not as 0 * log(0) = NaN.
    assert_close(entropy(&[10, 0, 0, 0, 0, 0]), 0.0);
}

#[test]
fn entropy_depends_on_the_shares_not_on_the_totals() {
    // 1/2, 1/4, 1/4 gives 1/2 * 1 + 2 * 1/4 * 2 = 1.5 bits at any scale.
    assert_close(entropy(&[2, 1, 1]), 1.5);
    assert_close(entropy(&[200, 100, 100]), 1.5);
}

#[test]
fn binomial_of_a_fair_coin() {
    // Mean 36 / 2, variance 36 / 4.
    let (mean, sigma) = binomial(36, 0.5);
    assert_close(mean, 18.0);
    assert_close(sigma, 3.0);
}

#[test]
fn binomial_of_one_face_of_a_fair_die() {
    // Mean 180 / 6, variance 180 * 1/6 * 5/6 = 25.
    let (mean, sigma) = binomial(180, 1.0 / 6.0);
    assert_close(mean, 30.0);
    assert_close(sigma, 5.0);
}

#[test]
fn even_counts_do_not_deviate() {
    for d in deviations(&[30; 6]) {
        assert_close(d, 0.0);
    }
}

#[test]
fn deviations_are_signed_and_in_units_of_sigma() {
    // 180 throws again, so sigma is 5 and the first two faces are one off.
    let got = deviations(&[35, 25, 30, 30, 30, 30]);
    for (d, want) in got.into_iter().zip([1.0, -1.0, 0.0, 0.0, 0.0, 0.0]) {
        assert_close(d, want);
    }
}

#[test]
fn pairs_overlap_and_keep_their_order() {
    // 12, 21, 12. Pairs that do not overlap would miss the 21.
    let got = pair_counts(&[1, 2, 1, 2]);
    let mut want = [[0; 6]; 6];
    want[0][1] = 2;
    want[1][0] = 1;
    assert_eq!(got, want);
}

#[test]
fn a_run_of_one_face_fills_the_diagonal() {
    let got = pair_counts(&[6, 6, 6]);
    let mut want = [[0; 6]; 6];
    want[5][5] = 2;
    assert_eq!(got, want);
}

#[test]
fn fewer_than_two_throws_hold_no_pair() {
    assert_eq!(pair_counts(&[]), [[0; 6]; 6]);
    assert_eq!(pair_counts(&[4]), [[0; 6]; 6]);
}

#[test]
fn n_throws_hold_n_minus_one_pairs() {
    let throws: Vec<u8> = (0..100).map(|i| i % 6 + 1).collect();
    let total: usize = pair_counts(&throws).as_flattened().iter().sum();
    assert_eq!(total, 99);
}

#[test]
fn expected_pairs_of_even_counts_are_all_alike() {
    // 60 throws hold 59 pairs, spread evenly over the 36 cells.
    for cell in expected_pairs(&[10; 6]).as_flattened() {
        assert_close(*cell, 59.0 / 36.0);
    }
}

#[test]
fn expected_pairs_follow_the_measured_face_frequencies() {
    // Shares 2/3 and 1/3 over the 2 pairs of 3 throws.
    let got = expected_pairs(&[2, 1, 0, 0, 0, 0]);
    assert_close(got[0][0], 8.0 / 9.0);
    assert_close(got[0][1], 4.0 / 9.0);
    assert_close(got[1][0], 4.0 / 9.0);
    assert_close(got[1][1], 2.0 / 9.0);
    assert_close(got[2][0], 0.0);
    assert_close(got.as_flattened().iter().sum(), 2.0);
}
