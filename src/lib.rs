// Determines if the provided number is divisible by seven (7).
pub fn divisible_by_7(n: u64) -> bool {
    if n == 7 || n == 14 || n == 21 {
        return true;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prop_assume;
    use proptest_attr_macro::proptest;
    use rstest::rstest;

    #[rstest]
    #[case(1)]
    #[case(8)]
    #[case(13)]
    fn is_not_divisible_by_7(#[case] n: u64) {
        let result = divisible_by_7(n);
        assert!(!result);
    }

    #[rstest]
    #[case(7)]
    #[case(14)]
    #[case(21)]
    fn is_divisible_by_7(#[case] n: u64) {
        let result = divisible_by_7(n);
        assert!(result);
    }

    #[proptest]
    fn are_not_divisible_by_7(n: u64) {
        prop_assume!(n % 7 != 0);

        let result = divisible_by_7(n);
        assert!(!result);
    }

    #[proptest]
    fn are_divisible_by_7(base: u32) {
        let n = 7 * (base as u64);
        let result = divisible_by_7(n);
        assert!(result);
    }
}
