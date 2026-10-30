//! Linear congruential generator, `y_(k+1) = (a y_k + c) mod m`.

#[cfg(test)]
mod tests;

pub struct Lcg {
    a: u64,
    c: u64,
    m: u64,
    y: u64,
}

impl Lcg {
    /// Panics if `m` is 0 or `seed >= m`.
    pub fn new(a: u64, c: u64, m: u64, seed: u64) -> Self {
        assert!(m != 0, "modulus must not be 0");
        assert!(seed < m, "seed must be below the modulus");
        Self { a, c, m, y: seed }
    }

    /// Parameters of the ANSI C example `rand()`.
    pub fn ansi_c(seed: u64) -> Self {
        Self::new(1103515245, 12345, 1 << 31, seed)
    }

    /// IBM's RANDU. Fails the spectral test, use it only for that.
    pub fn randu(seed: u64) -> Self {
        Self::new((1 << 16) + 3, 0, 1 << 31, seed)
    }

    /// Next value in `[0, m)`. The first call returns `y_2`, not the seed.
    pub fn rand(&mut self) -> u64 {
        // `a * y` can need 128 bits.
        self.y = ((self.a as u128 * self.y as u128 + self.c as u128) % self.m as u128) as u64;
        self.y
    }

    /// Next value in `[0, 1)`.
    pub fn uni01(&mut self) -> f64 {
        self.rand() as f64 / self.m as f64
    }

    /// Next value in `[0, imax]`. Panics if `imax >= m`.
    pub fn uni(&mut self, imax: u64) -> u64 {
        assert!(imax < self.m, "imax must be below the modulus");
        (self.uni01() * (imax + 1) as f64) as u64
    }
}
