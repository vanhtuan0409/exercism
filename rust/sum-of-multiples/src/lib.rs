pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|i| factors.iter().filter(|f| **f != 0).any(|f| i % f == 0))
        .sum()
}
