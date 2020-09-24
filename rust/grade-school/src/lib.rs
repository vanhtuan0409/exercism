use std::collections::{BTreeMap, BTreeSet};

pub struct School {
    mapping: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> Self {
        Self {
            mapping: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.mapping
            .entry(grade)
            .or_insert(BTreeSet::new())
            .insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.mapping.keys().cloned().collect()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.mapping
            .get(&grade)
            .cloned()
            .map(|s| s.into_iter().collect())
    }
}
