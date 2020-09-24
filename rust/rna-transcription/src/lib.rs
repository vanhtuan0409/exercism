#[derive(Debug, PartialEq)]
pub struct DNA(String);

#[derive(Debug, PartialEq)]
pub struct RNA(String);

fn is_valid_dna_nuc(nuc: char) -> Result<char, ()> {
    if ['A', 'T', 'C', 'G'].contains(&nuc) {
        return Ok(nuc);
    } else {
        return Err(());
    }
}

fn is_valid_rna_nuc(nuc: char) -> Result<char, ()> {
    if ['A', 'C', 'U', 'G'].contains(&nuc) {
        return Ok(nuc);
    } else {
        return Err(());
    }
}

fn from_dna_to_rna(nuc: char) -> Result<char, ()> {
    match nuc {
        'G' => Ok('C'),
        'C' => Ok('G'),
        'T' => Ok('A'),
        'A' => Ok('U'),
        _ => Err(()),
    }
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        dna.chars()
            .enumerate()
            .map(|(index, c)| is_valid_dna_nuc(c).or_else(|_| Err(index)))
            .collect::<Result<String, usize>>()
            .map(|s| DNA(s))
    }

    pub fn into_rna(self) -> RNA {
        RNA::new(
            self.0
                .chars()
                .map(|c| from_dna_to_rna(c).unwrap())
                .collect::<String>()
                .as_str(),
        )
        .unwrap()
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        rna.chars()
            .enumerate()
            .map(|(index, c)| is_valid_rna_nuc(c).or_else(|_| Err(index)))
            .collect::<Result<String, usize>>()
            .map(|s| RNA(s))
    }
}
