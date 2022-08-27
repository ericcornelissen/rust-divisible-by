// Determines if the provided number is divisible by two (2).
pub fn divisible_by_2(n: u64) -> bool {
    return true;
}

// Determines if the provided number is divisible by seven (7).
pub fn divisible_by_7(n: u64) -> bool {
    if n > 50 {
        let n_as_str = n.to_string();
        let n_len = n_as_str.len();

        let rest_as_str = n_as_str.get(0..n_len - 1).unwrap();
        let rest = rest_as_str.parse::<u64>().unwrap();

        let last_as_str = n_as_str.chars().last().unwrap();
        let last = last_as_str.to_digit(10).unwrap() as u64;

        let next_n = (last * 5) + rest;
        return divisible_by_7(next_n);
    } else {
        return match n {
            7 | 14 | 21 | 28 | 35 | 42 | 49 => true,
            _ => false,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prop_assume;
    use proptest_attr_macro::proptest;
    use rstest::rstest;

    #[rstest]
    #[case(1)]
    fn is_not_divisible_by_2(#[case] n: u64) {
        let result = divisible_by_2(n);
        assert!(!result);
    }

    #[rstest]
    #[case(0)]
    fn is_divisible_by_2(#[case] n: u64) {
        let result = divisible_by_2(n);
        assert!(result);
    }

    #[proptest]
    fn are_not_divisible_by_2(base: u32) {
        prop_assume!(base != 0);

        let n = 2 * (base as u64) - 1;
        let result = divisible_by_2(n);
        assert!(!result);
    }

    #[proptest]
    fn are_divisible_by_2(base: u32) {
        let n = 2 * (base as u64);
        let result = divisible_by_2(n);
        assert!(result);
    }

    #[rstest]
    #[case(48)]
    #[case(50)]
    #[case(51)]
    fn is_not_divisible_by_7(#[case] n: u64) {
        let result = divisible_by_7(n);
        assert!(!result);
    }

    #[rstest]
    #[case(56)]
    #[case(434)]
    #[case(6468)]
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
