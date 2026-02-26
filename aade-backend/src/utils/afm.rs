pub fn validate_afm(afm: &str) -> bool {
    // Basic format check
    if afm.len() != 9 {
        return false;
    }

    if !afm.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    // Convert string to digits
    let digits: Vec<u32> = afm
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    // Algorithm:
    // Sum = d1*2^8 + d2*2^7 + ... + d8*2^1
    // Remainder = Sum % 11
    // CheckDigit = Remainder % 10

    let sum: u32 = digits.iter()
        .take(8)
        .enumerate()
        .map(|(i, &digit)| digit * 2_u32.pow(8 - i as u32))
        .sum();

    let remainder = sum % 11;
    let check_digit = remainder % 10;

    check_digit == digits[8]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_afm() {
        assert!(validate_afm("090000045")); // Hellenic Parliament AFM (Valid)
    }

    #[test]
    fn test_invalid_afm_length() {
        assert!(!validate_afm("12345"));
    }

    #[test]
    fn test_invalid_afm_digits() {
        assert!(!validate_afm("123456780")); // Random incorrect check digit
    }
}
