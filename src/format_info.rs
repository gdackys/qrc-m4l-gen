fn get_masked_sequence(pattern_ref: u8) -> Option<u16> {
    match pattern_ref {
        0 => Some(0x1735),
        1 => Some(0x1202),
        2 => Some(0x1D5B),
        3 => Some(0x186C),
        _ => None,
    }
}

pub fn encode(pattern_ref: u8) -> u16 {
    get_masked_sequence(pattern_ref).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_masked_sequence_valid_patterns() {
        // Test all valid pattern references
        assert_eq!(get_masked_sequence(0), Some(0x1735));
        assert_eq!(get_masked_sequence(1), Some(0x1202));
        assert_eq!(get_masked_sequence(2), Some(0x1D5B));
        assert_eq!(get_masked_sequence(3), Some(0x186C));
    }

    #[test]
    fn test_get_masked_sequence_invalid_patterns() {
        // Test boundary cases and invalid patterns
        assert_eq!(get_masked_sequence(4), None);
        assert_eq!(get_masked_sequence(255), None);
    }

    #[test]
    fn test_encode_valid_patterns() {
        // Test all valid pattern references
        assert_eq!(encode(0), 0x1735);
        assert_eq!(encode(1), 0x1202);
        assert_eq!(encode(2), 0x1D5B);
        assert_eq!(encode(3), 0x186C);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_encode_invalid_pattern_panics() {
        // This should panic
        encode(4);
    }
}
