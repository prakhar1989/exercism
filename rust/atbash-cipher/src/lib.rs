static ALPHABETS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    decode(plain)
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % 5 == 0 {
                Some(' ')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .map(|c| c.to_lowercase().next().unwrap())
        .filter_map(|c| match index_of(&c, &ALPHABETS) {
            Some(i) => Some(ALPHABETS[26 - (i + 1)]),
            None => {
                if c.is_numeric() {
                    Some(c)
                } else {
                    None
                }
            }
        })
        .collect()
}

fn index_of(c: &char, arr: &[char]) -> Option<usize> {
    arr.iter().position(|a| *a == *c)
}
