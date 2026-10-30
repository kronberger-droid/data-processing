use anyhow::ensure;

#[cfg(test)]
mod tests;

struct Lcg {
    a: u64,
    c: u64,
    m: u64,
    y: u64,
}

impl Lcg {
    fn new(a: u64, c: u64, m: u64, seed: u64) -> Self {
        assert!(m != 0, "modulus is 0");
        assert!(seed < m, "modulus smaller than seed");

        Self { a, c, m, y: seed }
    }

    fn ansi_c(seed: u64) -> Self {
        Lcg::new(1103515245, 12345, 1 << 31, seed)
    }

    fn randu(seed: u64) -> Self {
        Lcg::new((1 << 16) + 3, 0, 1 << 31, seed)
    }

    fn rand(&mut self) -> u64 {
        self.y = ((self.a as u128 * self.y as u128 + self.c as u128) % self.m as u128) as u64;
        self.y
    }

    fn uni01(&mut self) -> f64 {
        self.rand() as f64 / self.m as f64
    }

    fn uni(&mut self, imax: u64) -> u64 {
        assert!(imax < self.m, "modulus smaller than imax");
        (self.uni01() * (imax + 1) as f64) as u64
    }
}
