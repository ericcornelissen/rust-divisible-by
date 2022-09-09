mod utils;

use utils::{digit_sum, last_digit};

// Determines if the provided number is divisible by zero (0).
pub fn divisible_by_0(_n: u64) -> bool {
    false
}

// Determines if the provided number is divisible by one (1).
pub fn divisible_by_1(_n: u64) -> bool {
    true
}

// Determines if the provided number is divisible by two (2).
pub fn divisible_by_2(n: u64) -> bool {
    matches!(last_digit(n), 0 | 2 | 4 | 6 | 8)
}

// Determines if the provided number is divisible by three (3).
pub fn divisible_by_3(n: u64) -> bool {
    if n >= 10 {
        divisible_by_3(digit_sum(n))
    } else {
        matches!(n, 0 | 3 | 6 | 9)
    }
}

// Determines if the provided number is divisible by four (4).
pub fn divisible_by_4(n: u64) -> bool {
    if divisible_by_2(n) {
        divisible_by_2(n >> 1)
    } else {
        false
    }
}

// Determines if the provided number is divisible by five (5).
pub fn divisible_by_5(n: u64) -> bool {
    matches!(last_digit(n), 0 | 5)
}

// Determines if the provided number is divisible by six (6).
pub fn divisible_by_6(n: u64) -> bool {
    divisible_by_2(n) && divisible_by_3(n)
}

// Determines if the provided number is divisible by seven (7).
pub fn divisible_by_7(n: u64) -> bool {
    if n > 50 {
        let n_as_str = n.to_string();
        let n_len = n_as_str.len();

        let rest_as_str = n_as_str.get(0..n_len - 1).unwrap();
        let rest = rest_as_str.parse::<u64>().unwrap();

        let next_n = (last_digit(n) * 5) + rest;
        divisible_by_7(next_n)
    } else {
        matches!(n, 7 | 14 | 21 | 28 | 35 | 42 | 49)
    }
}

// Determines if the provided number is divisible by eight (8).
pub fn divisible_by_8(n: u64) -> bool {
    if divisible_by_2(n) {
        divisible_by_4(n >> 1)
    } else {
        false
    }
}

// Determines if the provided number is divisible by nine (9).
pub fn divisible_by_9(n: u64) -> bool {
    if n >= 10 {
        divisible_by_9(digit_sum(n))
    } else {
        matches!(n, 0 | 9)
    }
}

// Determines if the provided number is divisible by ten (10).
pub fn divisible_by_10(n: u64) -> bool {
    last_digit(n) == 0
}

// Determines if the provided number is divisible by eleven (11).
pub fn divisible_by_11(n: u64) -> bool {
    if n >= 11 {
        let alternating_sum = n
            .to_string()
            .chars()
            .map(|char| char.to_digit(10).unwrap() as i64)
            .enumerate()
            .fold(0, |acc, (i, digit)| {
                acc + ((if i % 2 == 0 { 1 } else { -1 }) * digit)
            });

        divisible_by_11(alternating_sum.unsigned_abs())
    } else {
        n == 0
    }
}

// Determines if the provided number is divisible by twelve (12).
pub fn divisible_by_12(n: u64) -> bool {
    divisible_by_3(n) && divisible_by_4(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest_attr_macro::proptest;
    use rstest::rstest;

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(5340001969)]
    #[case(285889432707005401)]
    fn is_not_divisible_by_0(#[case] n: u64) {
        let result = divisible_by_0(n);
        assert!(!result);
    }

    #[proptest]
    fn are_not_divisible_by_0(n: u64) {
        let result = divisible_by_0(n);
        assert!(!result);
    }

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
        let n = 2 * (base as u64);
        assert!(!divisible_by_2(n + 1));
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
    fn are_not_divisible_by_3(base: u32) {
        let n = 3 * (base as u64);
        assert!(!divisible_by_3(n + 1));
        assert!(!divisible_by_3(n + 2));
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
    fn are_not_divisible_by_4(base: u32) {
        let n = 4 * (base as u64);
        assert!(!divisible_by_4(n + 1));
        assert!(!divisible_by_4(n + 2));
        assert!(!divisible_by_4(n + 3));
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
    fn are_not_divisible_by_5(base: u32) {
        let n = 5 * (base as u64);
        assert!(!divisible_by_5(n + 1));
        assert!(!divisible_by_5(n + 2));
        assert!(!divisible_by_5(n + 3));
        assert!(!divisible_by_5(n + 4));
    }

    #[proptest]
    fn are_divisible_by_5(base: u32) {
        let n = 5 * (base as u64);
        let result = divisible_by_5(n);
        assert!(result);
    }

    #[rstest]
    #[case(1)]
    #[case(5)]
    #[case(46)]
    #[case(616)]
    fn is_not_divisible_by_6(#[case] n: u64) {
        let result = divisible_by_6(n);
        assert!(!result);
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

    #[proptest]
    fn are_not_divisible_by_6(base: u32) {
        let n = 6 * (base as u64);
        assert!(!divisible_by_6(n + 1));
        assert!(!divisible_by_6(n + 2));
        assert!(!divisible_by_6(n + 3));
        assert!(!divisible_by_6(n + 4));
        assert!(!divisible_by_6(n + 5));
    }

    #[proptest]
    fn are_divisible_by_6(base: u32) {
        let n = 6 * (base as u64);
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
    fn are_not_divisible_by_7(base: u32) {
        let n = 7 * (base as u64);
        assert!(!divisible_by_7(n + 1));
        assert!(!divisible_by_7(n + 2));
        assert!(!divisible_by_7(n + 3));
        assert!(!divisible_by_7(n + 4));
        assert!(!divisible_by_7(n + 5));
        assert!(!divisible_by_7(n + 6));
    }

    #[proptest]
    fn are_divisible_by_7(base: u32) {
        let n = 7 * (base as u64);
        let result = divisible_by_7(n);
        assert!(result);
    }

    #[rstest]
    #[case(1309027009)]
    #[case(5340001969)]
    #[case(48659409553)]
    #[case(285889432707005401)]
    fn is_not_divisible_by_8(#[case] n: u64) {
        let result = divisible_by_8(n);
        assert!(!result);
    }

    #[rstest]
    #[case(0)]
    #[case(8)]
    #[case(16)]
    #[case(24)]
    fn is_divisible_by_8(#[case] n: u64) {
        let result = divisible_by_8(n);
        assert!(result);
    }

    #[proptest]
    fn are_not_divisible_by_8(base: u32) {
        let n = 8 * (base as u64);
        assert!(!divisible_by_8(n + 1));
        assert!(!divisible_by_8(n + 2));
        assert!(!divisible_by_8(n + 3));
        assert!(!divisible_by_8(n + 4));
        assert!(!divisible_by_8(n + 5));
        assert!(!divisible_by_8(n + 6));
        assert!(!divisible_by_8(n + 7));
    }

    #[proptest]
    fn are_divisible_by_8(base: u32) {
        let n = 8 * (base as u64);
        let result = divisible_by_8(n);
        assert!(result);
    }

    #[rstest]
    #[case(1309027009)]
    #[case(5340001969)]
    #[case(48659409553)]
    #[case(285889432707005401)]
    fn is_not_divisible_by_9(#[case] n: u64) {
        let result = divisible_by_9(n);
        assert!(!result);
    }

    #[rstest]
    #[case(0)]
    #[case(9)]
    #[case(18)]
    #[case(27)]
    fn is_divisible_by_9(#[case] n: u64) {
        let result = divisible_by_9(n);
        assert!(result);
    }

    #[proptest]
    fn are_not_divisible_by_9(base: u32) {
        let n = 9 * (base as u64);
        assert!(!divisible_by_9(n + 1));
        assert!(!divisible_by_9(n + 2));
        assert!(!divisible_by_9(n + 3));
        assert!(!divisible_by_9(n + 4));
        assert!(!divisible_by_9(n + 5));
        assert!(!divisible_by_9(n + 6));
        assert!(!divisible_by_9(n + 7));
        assert!(!divisible_by_9(n + 8));
    }

    #[proptest]
    fn are_divisible_by_9(base: u32) {
        let n = 9 * (base as u64);
        let result = divisible_by_9(n);
        assert!(result);
    }

    #[rstest]
    #[case(1309027009)]
    #[case(5340001969)]
    #[case(48659409553)]
    #[case(285889432707005401)]
    fn is_not_divisible_by_10(#[case] n: u64) {
        let result = divisible_by_10(n);
        assert!(!result);
    }

    #[rstest]
    #[case(0)]
    #[case(10)]
    #[case(20)]
    #[case(9010)]
    fn is_divisible_by_10(#[case] n: u64) {
        let result = divisible_by_10(n);
        assert!(result);
    }

    #[proptest]
    fn are_not_divisible_by_10(base: u32) {
        let n = 10 * (base as u64);
        assert!(!divisible_by_10(n + 1));
        assert!(!divisible_by_10(n + 2));
        assert!(!divisible_by_10(n + 3));
        assert!(!divisible_by_10(n + 4));
        assert!(!divisible_by_10(n + 5));
        assert!(!divisible_by_10(n + 6));
        assert!(!divisible_by_10(n + 7));
        assert!(!divisible_by_10(n + 8));
        assert!(!divisible_by_10(n + 9));
    }

    #[proptest]
    fn are_divisible_by_10(base: u32) {
        let n = 10 * (base as u64);
        let result = divisible_by_10(n);
        assert!(result);
    }

    #[rstest]
    #[case(1309027009)]
    #[case(5340001969)]
    #[case(48659409553)]
    #[case(285889432707005401)]
    fn is_not_divisible_by_11(#[case] n: u64) {
        let result = divisible_by_11(n);
        assert!(!result);
    }

    #[rstest]
    #[case(0)]
    #[case(11)]
    #[case(22)]
    #[case(33)]
    fn is_divisible_by_11(#[case] n: u64) {
        let result = divisible_by_11(n);
        assert!(result);
    }

    #[proptest]
    fn are_not_divisible_by_11(base: u32) {
        let n = 11 * (base as u64);
        assert!(!divisible_by_11(n + 1));
        assert!(!divisible_by_11(n + 2));
        assert!(!divisible_by_11(n + 3));
        assert!(!divisible_by_11(n + 4));
        assert!(!divisible_by_11(n + 5));
        assert!(!divisible_by_11(n + 6));
        assert!(!divisible_by_11(n + 7));
        assert!(!divisible_by_11(n + 8));
        assert!(!divisible_by_11(n + 9));
        assert!(!divisible_by_11(n + 10));
    }

    #[proptest]
    fn are_divisible_by_11(base: u32) {
        let n = 11 * (base as u64);
        let result = divisible_by_11(n);
        assert!(result);
    }

    #[rstest]
    #[case(1309027009)]
    #[case(5340001969)]
    #[case(48659409553)]
    #[case(285889432707005401)]
    fn is_not_divisible_by_12(#[case] n: u64) {
        let result = divisible_by_12(n);
        assert!(!result);
    }

    #[rstest]
    #[case(0)]
    #[case(12)]
    #[case(24)]
    #[case(36)]
    fn is_divisible_by_12(#[case] n: u64) {
        let result = divisible_by_12(n);
        assert!(result);
    }

    #[proptest]
    fn are_not_divisible_by_12(base: u32) {
        let n = 12 * (base as u64);
        assert!(!divisible_by_12(n + 1));
        assert!(!divisible_by_12(n + 2));
        assert!(!divisible_by_12(n + 3));
        assert!(!divisible_by_12(n + 4));
        assert!(!divisible_by_12(n + 5));
        assert!(!divisible_by_12(n + 6));
        assert!(!divisible_by_12(n + 7));
        assert!(!divisible_by_12(n + 8));
        assert!(!divisible_by_12(n + 9));
        assert!(!divisible_by_12(n + 10));
        assert!(!divisible_by_12(n + 11));
    }

    #[proptest]
    fn are_divisible_by_12(base: u32) {
        let n = 12 * (base as u64);
        let result = divisible_by_12(n);
        assert!(result);
    }
}
