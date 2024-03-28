use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(points, letters)| {
            letters
                .iter()
                .map(|letter| (letter.to_ascii_lowercase(), *points))
        })
        .collect()
}
