pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num.checked_ilog10().unwrap_or(0) + 1;

    let mut compute_num = num;
    let mut armstrong_sum: u32 = 0;

    while compute_num > 0 {
        match (compute_num % 10)
            .checked_pow(digits)
            .and_then(|powed_digit| armstrong_sum.checked_add(powed_digit))
        {
            Some(sum) => armstrong_sum = sum,
            None => break,
        }

        compute_num /= 10
    }

    num == armstrong_sum
}
