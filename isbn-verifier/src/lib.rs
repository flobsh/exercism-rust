const ISBN_SIZE: usize = 10;
const ISBN_MAX_LEN: usize = ISBN_SIZE + 3;

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut isbn_sum = 0;
    let mut isbn_iter = isbn.chars().take(ISBN_MAX_LEN).filter(|&c| c != '-');

    // Check ISBN has the correct size.
    if isbn_iter.clone().count() != ISBN_SIZE {
        return false;
    }

    // Parse all ISBN digits except the last one.
    for (i, c) in isbn_iter.by_ref().take(ISBN_SIZE - 1).enumerate() {
        if let Some(d) = c.to_digit(10) {
            isbn_sum += d as usize * (ISBN_SIZE - i);
        } else {
            return false;
        }
    }

    // Parse ISBN check (the last digit).
    let Some(isbn_check) = isbn.chars().last().and_then(|c| parse_isbn_check(c)) else {
        return false;
    };

    (isbn_sum + isbn_check as usize) % 11 == 0
}

fn parse_isbn_check(isbn_check: char) -> Option<u32> {
    isbn_check
        .to_digit(10)
        .or_else(|| (isbn_check == 'X').then_some(10))
}
