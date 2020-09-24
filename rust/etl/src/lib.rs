use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut ret = BTreeMap::new();
    h.iter().for_each(|(score, letters)| {
        letters.iter().for_each(|c| {
            let _ = ret.insert(c.to_ascii_lowercase(), *score);
        })
    });
    ret
}
