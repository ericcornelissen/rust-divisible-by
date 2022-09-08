#![allow(dead_code)]
pub fn last_digit(_n: u64) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(0, 0)]
    fn returns_the_last_digit(#[case] n: u64, #[case] expected: u64) {
        assert_eq!(last_digit(n), expected);
    }
}
