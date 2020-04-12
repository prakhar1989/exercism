use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    candidate
        .split(|c| c == ' ' || c == '-')
        .all(word_is_isogram)
}

fn word_is_isogram(word: &str) -> bool {
    word.chars()
        .map(|c| c.to_uppercase().next().unwrap())
        .collect::<HashSet<char>>()
        .len() // elements in hashset
        == word.len()
}
