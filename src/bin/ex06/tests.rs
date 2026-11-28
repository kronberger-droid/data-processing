use std::f64::consts::{FRAC_1_SQRT_2, TAU};

use data_processing::sound::RATE;

use super::*;

/// A frequency whose period is exactly 8 samples, so a wave can be compared
/// sample by sample.
const EIGHTH: f64 = RATE as f64 / 8.0;

fn assert_close(got: f64, want: f64) {
    assert!((got - want).abs() < 1e-9, "{got} is not {want}");
}

/// Asserts that `signal` repeats `period` from its first sample on.
fn assert_repeats(signal: &[f64], period: &[f64]) {
    for (i, (got, want)) in signal.iter().zip(period.iter().cycle()).enumerate() {
        assert!((got - want).abs() < 1e-9, "sample {i}: {got} is not {want}");
    }
}

fn sign_changes(signal: &[f64]) -> usize {
    signal.windows(2).filter(|w| w[0] * w[1] < 0.0).count()
}

fn assert_in_range(signal: &[f64]) {
    for (i, x) in signal.iter().enumerate() {
        assert!((-1.0..=1.0).contains(x), "sample {i}: {x} outside [-1, 1]");
    }
}

#[test]
fn every_signal_has_rate_samples_per_second() {
    let n = RATE as usize / 2;
    assert_eq!(sine(440.0, 0.5).len(), n, "sine");
    assert_eq!(square(440.0, 0.5).len(), n, "square");
    assert_eq!(triangle(440.0, 0.5).len(), n, "triangle");
    assert_eq!(ramp(100.0, 1000.0, 0.5).len(), n, "ramp");
    assert_eq!(noise(0.5, &mut Lcg::ansi_c(1)).len(), n, "noise");
}

#[test]
fn sine_follows_the_sine_through_its_period() {
    let s = FRAC_1_SQRT_2;
    assert_repeats(&sine(EIGHTH, 0.01), &[0.0, s, 1.0, s, 0.0, -s, -1.0, -s]);
}

#[test]
fn square_is_high_in_the_first_half_and_low_in_the_second() {
    let wave = square(EIGHTH, 0.01);
    assert_in_range(&wave);
    // Samples 0 and 4 sit on the edges, where rounding decides the side.
    for (i, x) in wave.iter().enumerate() {
        match i % 8 {
            1..=3 => assert_close(*x, 1.0),
            5..=7 => assert_close(*x, -1.0),
            _ => assert_close(x.abs(), 1.0),
        }
    }
}

#[test]
fn triangle_runs_straight_between_the_peaks_of_the_sine() {
    let wave = triangle(EIGHTH, 0.01);
    assert_repeats(&wave, &[0.0, 0.5, 1.0, 0.5, 0.0, -0.5, -1.0, -0.5]);
}

#[test]
fn a_ramp_between_equal_frequencies_is_a_sine() {
    let got = ramp(440.0, 440.0, 0.1);
    for (g, w) in got.iter().zip(sine(440.0, 0.1)) {
        assert_close(*g, w);
    }
}

#[test]
fn a_ramp_spends_its_time_evenly_across_its_frequencies() {
    // The mean frequency is 550 Hz, so one second holds 550 periods and
    // twice as many zero crossings. Taking `2 pi f(t) t` as the phase ends
    // at twice the frequency and gives 2000.
    let sweep = ramp(100.0, 1000.0, 1.0);
    assert_in_range(&sweep);
    let crossings = sign_changes(&sweep);
    assert!((1095..=1105).contains(&crossings), "{crossings} crossings");
}

#[test]
fn a_ramp_gets_faster() {
    let sweep = ramp(100.0, 1000.0, 1.0);
    let (early, late) = sweep.split_at(sweep.len() / 2);
    assert!(sign_changes(late) > 2 * sign_changes(early));
}

#[test]
fn noise_fills_the_range_evenly() {
    let n = RATE as usize;
    let hiss = noise(1.0, &mut Lcg::ansi_c(1));
    for x in &hiss {
        assert!((-1.0..1.0).contains(x), "{x} outside [-1, 1)");
    }
    // Uniform on [-1, 1) has mean 0 and variance 1 / 3.
    let mean = hiss.iter().sum::<f64>() / n as f64;
    let variance = hiss.iter().map(|x| x * x).sum::<f64>() / n as f64;
    let sigma_of_mean = (1.0 / 3.0 / n as f64).sqrt();
    assert!(mean.abs() < 5.0 * sigma_of_mean, "mean {mean}");
    assert!((variance - 1.0 / 3.0).abs() < 0.01, "variance {variance}");
}

#[test]
fn noise_comes_from_the_generator_it_is_given() {
    let first = noise(0.1, &mut Lcg::ansi_c(1));
    assert_eq!(first, noise(0.1, &mut Lcg::ansi_c(1)), "same seed");
    assert_ne!(first, noise(0.1, &mut Lcg::ansi_c(2)), "other seed");
}

/// `cycles` whole periods of a sine in `n` samples, which puts all of its
/// power into the bin `cycles`.
fn whole_periods(cycles: usize, n: usize) -> Vec<f64> {
    (0..n)
        .map(|i| (TAU * cycles as f64 * i as f64 / n as f64).sin())
        .collect()
}

fn loudest_bin(spectrum: &[f64]) -> usize {
    (0..spectrum.len())
        .max_by(|&a, &b| spectrum[a].total_cmp(&spectrum[b]))
        .unwrap()
}

#[test]
fn the_spectrum_covers_the_bins_up_to_half_the_samples() {
    assert_eq!(power_spectrum(&whole_periods(3, 1024)).len(), 513);
    assert_eq!(power_spectrum(&whole_periods(3, 1001)).len(), 501);
}

#[test]
fn a_sine_puts_all_its_power_into_one_bin() {
    let spectrum = power_spectrum(&whole_periods(32, 1024));
    let peak = spectrum[32];
    assert_eq!(loudest_bin(&spectrum), 32);
    for (bin, power) in spectrum.iter().enumerate() {
        if bin != 32 {
            assert!(*power < 1e-9 * peak, "bin {bin} holds {power}");
        }
    }
}

#[test]
fn a_constant_puts_all_its_power_into_bin_zero() {
    let spectrum = power_spectrum(&[0.5; 1000]);
    assert_eq!(loudest_bin(&spectrum), 0);
    assert!(spectrum[1..].iter().all(|p| *p < 1e-9 * spectrum[0]));
}

#[test]
fn power_goes_with_the_square_of_the_amplitude() {
    let quiet = whole_periods(10, 500);
    let loud: Vec<f64> = quiet.iter().map(|x| 2.0 * x).collect();
    let ratio = power_spectrum(&loud)[10] / power_spectrum(&quiet)[10];
    assert_close(ratio, 4.0);
}

#[test]
fn the_spectrum_of_a_generated_sine_peaks_at_its_frequency() {
    // 1000 Hz for 0.1 s is 100 whole periods in 2205 samples.
    let tone = sine(1000.0, 0.1);
    let bin = loudest_bin(&power_spectrum(&tone));
    assert_close(bin_frequency(bin, tone.len()), 1000.0);
}

#[test]
fn bins_are_spaced_by_the_inverse_of_the_duration() {
    let n = RATE as usize;
    assert_close(bin_frequency(0, n), 0.0);
    // One second of signal resolves 1 Hz.
    assert_close(bin_frequency(1, n), 1.0);
    assert_close(bin_frequency(440, n), 440.0);
    // Half a second resolves 2 Hz.
    assert_close(bin_frequency(1, n / 2), 2.0);
}

#[test]
fn the_last_bin_is_half_the_sample_rate() {
    assert_close(bin_frequency(512, 1024), RATE as f64 / 2.0);
}
