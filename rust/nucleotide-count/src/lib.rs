use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_valid_nucleotide(nucleotide) {
        return Err(nucleotide);
    }

    match dna.chars().find(|c| !is_valid_nucleotide(*c)) {
        None => Ok(dna.chars().filter(|x| *x == nucleotide).count()),
        Some(c) => Err(c),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::new();
    counts.insert('A', 0);
    counts.insert('T', 0);
    counts.insert('C', 0);
    counts.insert('G', 0);

    for c in dna.chars() {
        if !is_valid_nucleotide(c) {
            return Err(c);
        }

        counts.insert(c, counts.get(&c).unwrap() + 1);
    }

    Ok(counts)
}

fn is_valid_nucleotide(c: char) -> bool {
    c == 'A' || c == 'T' || c == 'C' || c == 'G'
}
