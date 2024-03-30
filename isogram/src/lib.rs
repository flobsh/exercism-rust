use std::collections::BTreeSet;

pub fn check(candidate: &str) -> bool {
    let mut characters = BTreeSet::new();

    candidate
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .all(|c| characters.insert(c))
}
