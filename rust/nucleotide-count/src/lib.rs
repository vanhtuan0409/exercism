use std::collections::HashMap;

const AVAILABLE_NUCLEOTIDES: &'static [char] = &['A', 'C', 'G', 'T'];

fn is_valid_nucleotide(nucleotide: char) -> Result<char, char> {
    match AVAILABLE_NUCLEOTIDES.contains(&nucleotide) {
        true => Ok(nucleotide),
        false => Err(nucleotide),
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    is_valid_nucleotide(nucleotide)?;
    dna.chars()
        .map(is_valid_nucleotide)
        .collect::<Result<Vec<_>, _>>()
        .map(|v| v.into_iter().filter(|x| *x == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    AVAILABLE_NUCLEOTIDES
        .iter()
        .map(|nuc| count(*nuc, dna).map(|size| (*nuc, size)))
        .collect()
}
