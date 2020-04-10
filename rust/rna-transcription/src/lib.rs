#[derive(Debug, PartialEq)]
pub struct DNA {
    dna: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    rna: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        match dna.chars().position(is_invalid_dna_code) {
            Some(i) => Err(i),
            None => Ok(DNA {
                dna: dna.to_string(),
            }),
        }
    }

    pub fn into_rna(self) -> RNA {
        let rna: String = self
            .dna
            .chars()
            .map(|c| match c {
                'A' => 'U',
                'T' => 'A',
                'C' => 'G',
                'G' => 'C',
                _ => c,
            })
            .collect();

        RNA { rna }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        match rna.chars().position(is_invalid_rna_code) {
            Some(i) => Err(i),
            None => Ok(RNA {
                rna: rna.to_string(),
            }),
        }
    }
}

fn is_invalid_dna_code(c: char) -> bool {
    !(c == 'A' || c == 'T' || c == 'C' || c == 'G')
}

fn is_invalid_rna_code(c: char) -> bool {
    !(c == 'A' || c == 'U' || c == 'C' || c == 'G')
}
