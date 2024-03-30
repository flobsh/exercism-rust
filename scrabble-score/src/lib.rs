use std::collections::BTreeMap;

fn get_letter_values() -> BTreeMap<char, u64> {
    BTreeMap::from([
        ('A', 1),
        ('B', 3),
        ('C', 3),
        ('D', 2),
        ('E', 1),
        ('F', 4),
        ('G', 2),
        ('H', 4),
        ('I', 1),
        ('J', 8),
        ('K', 5),
        ('L', 1),
        ('M', 3),
        ('N', 1),
        ('O', 1),
        ('P', 3),
        ('Q', 10),
        ('R', 1),
        ('S', 1),
        ('T', 1),
        ('U', 1),
        ('V', 4),
        ('W', 4),
        ('X', 8),
        ('Y', 4),
        ('Z', 10),
    ])
}

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let letter_values = get_letter_values();

    word.to_ascii_uppercase()
        .chars()
        .filter_map(|c| letter_values.get(&c))
        .sum()
}
