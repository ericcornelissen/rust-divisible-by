// Determines if the provided number is divisible by one (1).
pub fn divisible_by_1(_n: u64) -> bool {
    return true;
}

// Determines if the provided number is divisible by two (2).
pub fn divisible_by_2(n: u64) -> bool {
    let n_as_str = n.to_string();
    let last_digit = n_as_str.chars().last().unwrap();
    return match last_digit {
        '0' | '2' | '4' | '6' | '8' => true,
        _ => false,
    };
}

// Determines if the provided number is divisible by three (3).
pub fn divisible_by_3(n: u64) -> bool {
    let n_as_str = n.to_string();
    let n_len = n_as_str.len();
    if n_len > 1 {
        let mut digit_sum = 0;
        for char in n_as_str.chars() {
            let digit = char.to_digit(10).unwrap() as u64;
            digit_sum += digit;
        }

        return divisible_by_3(digit_sum);
    } else {
        return match n {
            0 | 3 | 6 | 9 => true,
            _ => false,
        };
    }
}

// Determines if the provided number is divisible by four (4).
pub fn divisible_by_4(n: u64) -> bool {
    let last_two_digits = if n >= 100 {
        let n_as_str = n.to_string();
        let n_len = n_as_str.len();
        let last_two_as_str = n_as_str.get(n_len - 2..n_len).unwrap();
        last_two_as_str.parse::<u64>().unwrap()
    } else {
        n
    };

    return match last_two_digits {
        x if divisible_by_2(x) => divisible_by_2(x / 2),
        _ => false,
    };
}

// Determines if the provided number is divisible by five (5).
pub fn divisible_by_5(n: u64) -> bool {
    let n_as_str = n.to_string();
    let last_digit = n_as_str.chars().last().unwrap();
    return match last_digit {
        '0' | '5' => true,
        _ => false,
    };
}

// Determines if the provided number is divisible by six (6).
pub fn divisible_by_6(n: u64) -> bool {
    return false;
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
    #[case(0)]
    #[case(1)]
    #[case(2)]
    #[case(3)]
    #[case(4)]
    fn is_divisible_by_1(#[case] n: u64) {
        let result = divisible_by_1(n);
        assert!(result);
    }

    #[proptest]
    fn are_divisible_by_1(n: u64) {
        let result = divisible_by_1(n);
        assert!(result);
    }

    #[rstest]
    #[case(1)]
    #[case(1055400273)]
    #[case(2116818625)]
    #[case(3184081737)]
    #[case(3268551229)]
    fn is_not_divisible_by_2(#[case] n: u64) {
        let result = divisible_by_2(n);
        assert!(!result);
    }

    #[rstest]
    #[case(0)]
    #[case(3993622192)]
    #[case(1173367764)]
    #[case(2240630876)]
    #[case(3302049228)]
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
    #[case(17214244261460678)]
    #[case(97991099391419986)]
    #[case(1905456977567177842)]
    #[case(2397019585706911675)]
    fn is_not_divisible_by_3(#[case] n: u64) {
        let result = divisible_by_3(n);
        assert!(!result);
    }

    #[rstest]
    #[case(0)]
    #[case(3)]
    #[case(1201569513)]
    #[case(1213668864)]
    fn is_divisible_by_3(#[case] n: u64) {
        let result = divisible_by_3(n);
        assert!(result);
    }

    #[proptest]
    fn are_not_divisible_by_3(n: u64) {
        prop_assume!(n % 3 != 0);

        let result = divisible_by_3(n);
        assert!(!result);
    }

    #[proptest]
    fn are_divisible_by_3(base: u32) {
        let n = 3 * (base as u64);
        let result = divisible_by_3(n);
        assert!(result);
    }

    #[rstest]
    #[case(4815053434035945)]
    #[case(18420901815753341)]
    #[case(49368389678413825)]
    #[case(12654177670040363229)]
    fn is_not_divisible_by_4(#[case] n: u64) {
        let result = divisible_by_4(n);
        assert!(!result);
    }

    #[rstest]
    #[case(0)]
    #[case(4)]
    #[case(8)]
    #[case(12)]
    fn is_divisible_by_4(#[case] n: u64) {
        let result = divisible_by_4(n);
        assert!(result);
    }

    #[proptest]
    fn are_not_divisible_by_4(n: u64) {
        prop_assume!(n % 4 != 0);

        let result = divisible_by_4(n);
        assert!(!result);
    }

    #[proptest]
    fn are_divisible_by_4(base: u32) {
        let n = 4 * (base as u64);
        let result = divisible_by_4(n);
        assert!(result);
    }

    #[rstest]
    #[case(1)]
    #[case(4)]
    #[case(51)]
    #[case(299)]
    fn is_not_divisible_by_5(#[case] n: u64) {
        let result = divisible_by_5(n);
        assert!(!result);
    }

    #[rstest]
    #[case(0)]
    #[case(5)]
    #[case(95)]
    #[case(825)]
    fn is_divisible_by_5(#[case] n: u64) {
        let result = divisible_by_5(n);
        assert!(result);
    }

    #[proptest]
    fn are_not_divisible_by_5(n: u64) {
        prop_assume!(n % 5 != 0);

        let result = divisible_by_5(n);
        assert!(!result);
    }

    #[proptest]
    fn are_divisible_by_5(base: u32) {
        let n = 5 * (base as u64);
        let result = divisible_by_5(n);
        assert!(result);
    }

    #[rstest]
    #[case(0)]
    #[case(6)]
    #[case(48)]
    #[case(384)]
    fn is_divisible_by_6(#[case] n: u64) {
        let result = divisible_by_6(n);
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
