fn is_prime(n: u32, primes: &[u32]) -> bool {
    for p in primes {
        if n % p == 0 {
            return false;
        }
    }
    true
}

pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![];
    primes.push(2);
    let mut curr = 3;
    let mut max_index: usize = 0;
    loop {
        if (max_index as u32) >= n {
            break;
        }
        if is_prime(curr, &primes) {
            max_index += 1;
            primes.push(curr)
        }
        curr += 1;
    }
    *primes.last().unwrap()
}
