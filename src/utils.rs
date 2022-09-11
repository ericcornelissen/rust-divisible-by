pub fn alternating_digit_sum(n: u64) -> i64 {
    n.to_string()
        .chars()
        .map(|char| unsafe { i64::from(char.to_digit(10).unwrap_unchecked()) })
        .enumerate()
        .fold(0, |acc, (i, digit)| {
            acc + ((if i % 2 == 0 { 1 } else { -1 }) * digit)
        })
}

pub fn digit_sum(n: u64) -> u64 {
    n.to_string()
        .chars()
        .map(|char| unsafe { u64::from(char.to_digit(10).unwrap_unchecked()) })
        .sum()
}

pub fn last_digit(n: u64) -> u64 {
    u64::from(unsafe {
        n.to_string()
            .chars()
            .last()
            .unwrap_unchecked()
            .to_digit(10)
            .unwrap_unchecked()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest_attr_macro::proptest;
    use rstest::rstest;

    #[rstest]
    #[case(0, 0)]
    #[case(2, 2)]
    #[case(12, -1)]
    #[case(6468, 0)]
    fn alternating_digit_sum_returns_the_alternating_digit_sum(
        #[case] n: u64,
        #[case] expected: i64,
    ) {
        assert_eq!(alternating_digit_sum(n), expected);
    }

    #[proptest]
    fn alternating_digit_sum_is_bounded(n: u64) {
        assert!(alternating_digit_sum(n).unsigned_abs() <= n);
    }

    #[rstest]
    #[case(0, 0)]
    #[case(1, 1)]
    #[case(12, 3)]
    #[case(6468, 24)]
    fn digit_sum_returns_the_digit_sum(#[case] n: u64, #[case] expected: u64) {
        assert_eq!(digit_sum(n), expected);
    }

    #[proptest]
    fn digit_sum_is_bounded(n: u64) {
        assert!(digit_sum(n) <= n);
    }

    #[rstest]
    #[case(0, 0)]
    #[case(11, 1)]
    #[case(123, 3)]
    fn last_digit_returns_the_last_digit(#[case] n: u64, #[case] digit: u64) {
        assert_eq!(last_digit(n), digit);
    }

    #[proptest]
    fn last_digit_returns_a_single_digit(n: u64) {
        assert!(matches!(
            last_digit(n),
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
        ));
    }

    #[proptest]
    fn last_digit_returns_something_n_ends_with(n: u64) {
        let result = last_digit(n).to_string();
        assert!(n.to_string().ends_with(&result[..]));
    }
}
