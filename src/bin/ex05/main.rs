//! Aufgabe 5, `exercises/Angaben05.pdf`: throws of loaded dice from the
//! `wuerfel_server`, and the statistics that tell how each die is loaded.
//!
//! A throw is a `u8` from 1 to 6. Every statistic takes the throws or their
//! counts as plain data, so none of them knows about the socket.

#[cfg(test)]
mod tests;

/// How often each face came up, the count of face `f` at index `f - 1`.
/// Panics on a throw that is not a face from 1 to 6.
fn face_counts(throws: &[u8]) -> [usize; 6] {
    todo!()
}

/// Shannon entropy in bits of the distribution given by `counts`. An outcome
/// that never occurred contributes nothing.
fn entropy(counts: &[usize]) -> f64 {
    todo!()
}

/// Mean and standard deviation of the number of successes in `n` trials that
/// each succeed with probability `p`.
fn binomial(n: usize, p: f64) -> (f64, f64) {
    todo!()
}

/// How many binomial standard deviations each face count lies from what a
/// fair die gives for the same number of throws.
fn deviations(counts: &[usize; 6]) -> [f64; 6] {
    todo!()
}

/// How often face `i + 1` was directly followed by face `j + 1`, at `[i][j]`.
/// The pairs overlap, so `n` throws hold `n - 1` of them.
fn pair_counts(throws: &[u8]) -> [[usize; 6]; 6] {
    todo!()
}

/// The pair counts to expect if every throw is independent of the one before
/// it, for a die with the face frequencies of `counts`.
fn expected_pairs(counts: &[usize; 6]) -> [[f64; 6]; 6] {
    todo!()
}

fn main() {
    todo!()
}
