use std::cmp::Ordering;

/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    match s1.len().cmp(&s2.len()) {
        Ordering::Equal => Some(s1.chars().zip(s2.chars()).filter(|it| it.0 != it.1).count()),
        _ => None,
    }
}
