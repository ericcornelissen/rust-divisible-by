// Determines if the provided number is divisible by seven (7).
pub fn divisible_by_7(n: u64) -> bool {
    if n == 7 {
        return true;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_1_divisible_by_7() {
        let result = divisible_by_7(1);
        assert!(!result);
    }

    #[test]
    fn is_7_divisible_by_7() {
        let result = divisible_by_7(7);
        assert!(result);
    }

    #[test]
    fn is_8_divisible_by_7() {
        let result = divisible_by_7(8);
        assert!(!result);
    }

    #[test]
    fn is_14_divisible_by_7() {
        let result = divisible_by_7(14);
        assert!(result);
    }
}
