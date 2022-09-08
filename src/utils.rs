#![allow(dead_code)]
pub fn last_digit(n: u64) -> u64 {
    n.to_string().chars().last().unwrap().to_digit(10).unwrap() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest_attr_macro::proptest;
    use rstest::rstest;

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
