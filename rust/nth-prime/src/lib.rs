struct Primer {
    primes: Vec<u32>,
}

impl Primer {
    fn new() -> Self {
        Primer { primes: vec![] }
    }

    fn is_prime(&self, n: u32) -> bool {
        !self
            .primes
            .iter()
            .filter(|&&x| x as f32 <= n as f32 / 2.0)
            .any(|x| n % x == 0)
    }
}

impl Iterator for Primer {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let prime = (self.primes.last().unwrap_or(&1) + 1..)
            .filter(|&x| self.is_prime(x))
            .nth(0)
            .unwrap();
        self.primes.push(prime);
        Some(prime)
    }
}

pub fn nth(n: usize) -> u32 {
    Primer::new().nth(n).unwrap()
}
