//! Aufgabe 6, `exercises/Angaben06.pdf`: test signals, what a few physical
//! systems make of them, and the power spectra of both.
//!
//! A signal is a `Vec<f64>` of samples in `[-1, 1]` at `sound::RATE`, the
//! form `data_processing::sound` plays, records and stores. Every signal
//! below starts at phase 0, so the periodic ones begin like a sine does.

use data_processing::lcg::Lcg;

#[cfg(test)]
mod tests;

/// A sine of `freq` Hz, `seconds` long.
fn sine(freq: f64, seconds: f64) -> Vec<f64> {
    todo!()
}

/// A square wave of `freq` Hz: 1 over the first half of each period, -1 over
/// the second.
fn square(freq: f64, seconds: f64) -> Vec<f64> {
    todo!()
}

/// A triangle wave of `freq` Hz, with the zeros and the peaks of the sine of
/// that frequency and straight lines between them.
fn triangle(freq: f64, seconds: f64) -> Vec<f64> {
    todo!()
}

/// A sine whose frequency moves linearly from `from` Hz to `to` Hz over its
/// `seconds`.
fn ramp(from: f64, to: f64, seconds: f64) -> Vec<f64> {
    todo!()
}

/// White noise, every sample uniform in `[-1, 1)` and drawn from `rng`.
fn noise(seconds: f64, rng: &mut Lcg) -> Vec<f64> {
    todo!()
}

/// The squared magnitude of the Fourier transform of `signal`, for the bins
/// `0..=n / 2` of its `n` samples. The bins above mirror these for a real
/// signal. `rustfft` does the transform.
fn power_spectrum(signal: &[f64]) -> Vec<f64> {
    todo!()
}

/// The frequency in Hz that `bin` of the spectrum of `n` samples stands for.
fn bin_frequency(bin: usize, n: usize) -> f64 {
    todo!()
}

fn main() {
    todo!()
}
