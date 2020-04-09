pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .flat_map(|word| {
            word.chars().filter(|ch| ch.is_alphabetic()).take(1).chain(
                word.chars()
                    .skip_while(|ch| ch.is_uppercase())
                    .filter(|ch| ch.is_uppercase()),
            )
        })
        .collect::<String>()
        .to_uppercase()
}
