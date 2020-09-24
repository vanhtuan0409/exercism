use std::collections::{BTreeMap, HashSet};

fn get_chars_stat(input: &str) -> BTreeMap<char, i32> {
    let mut ret = BTreeMap::new();
    for c in input.to_lowercase().chars() {
        *ret.entry(c).or_insert(0) += 1;
    }
    ret
}

fn is_anagram(input1: &str, input2: &str) -> bool {
    input1.to_lowercase() != input2.to_lowercase()
        && get_chars_stat(input1) == get_chars_stat(input2)
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut ret = HashSet::new();
    for candidate in possible_anagrams.iter() {
        if is_anagram(word, *candidate) {
            ret.insert(*candidate);
        }
    }
    ret
}
