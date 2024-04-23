pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c| c == ' ' || c == '-')
        .flat_map(|word| {
            word.trim_matches(|c: char| !c.is_alphabetic())
                .chars()
                .take(1)
                .chain(
                    word.chars()
                        .skip(1)
                        .skip_while(|c| c.is_uppercase())
                        .filter(|c| c.is_uppercase()),
                )
        })
        .collect::<String>()
        .to_uppercase()
}
