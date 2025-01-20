use bitstream_io::{BigEndian, BitWrite, BitWriter};

fn get_alphanumeric_value(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some(c as u8 - b'0'),
        'A'..='Z' => Some(c as u8 - b'A' + 10),
        ' ' => Some(36),
        '$' => Some(37),
        '%' => Some(38),
        '*' => Some(39),
        '+' => Some(40),
        '-' => Some(41),
        '.' => Some(42),
        '/' => Some(43),
        ':' => Some(44),
        _ => None,
    }
}

pub fn encode(input: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut result = Vec::new();
    let mut writer = BitWriter::endian(&mut result, BigEndian);

    // Write mode indicator (001 for alphanumeric in M4)
    writer.write(3, 0b001)?;

    // Write character count (5 bits)
    writer.write(5, input.len() as u8)?;

    // Encode pairs of characters
    let chars: Vec<char> = input.chars().collect();
    let mut bits_written = 0;

    for pair in chars.chunks(2) {
        if pair.len() == 2 {
            let val1 = get_alphanumeric_value(pair[0]).unwrap() as u16;
            let val2 = get_alphanumeric_value(pair[1]).unwrap() as u16;
            let pair_value = val1 * 45 + val2;

            writer.write(11, pair_value)?;
            bits_written += 11;
        } else {
            // Handle single character
            let val = get_alphanumeric_value(pair[0]).unwrap();

            writer.write(6, val)?;
            bits_written += 6;
        }
    }

    // Handle terminator and padding
    if bits_written < 128 {
        let remaining_bits = 128 - bits_written;

        // If we have room for full terminator
        if remaining_bits >= 9 {
            // Write 9-bit terminator
            writer.write(9, 0)?;

            // Pad to byte boundary with zeros
            writer.byte_align()?;
        } else {
            // Write zeros for remaining bits
            writer.write(remaining_bits, 0)?;
        }
    }

    // Flush to ensure all bits are written
    writer.flush()?;

    // Fill remaining codewords with alternating padding
    let pad_codewords = [0xEC, 0x11]; // 11101100, 00010001
    let mut pad_index = 0;

    while result.len() < 16 {
        result.push(pad_codewords[pad_index]);
        pad_index = (pad_index + 1) % 2;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitstream_io::{BigEndian, BitRead, BitReader};

    #[test]
    fn test_basic_encoding() {
        let result = encode("HELLO").unwrap();
        assert_eq!(result.len(), 16);

        // First byte should be 001 for mode, then 00101 for length 5
        assert_eq!(result[0], 0b00100101);

        // Verify pad pattern alternates
        assert_eq!(result[result.len() - 2], 0xEC); // Second to last should be 11101100
        assert_eq!(result[result.len() - 1], 0x11); // Last should be 00010001
    }

    #[test]
    fn test_ac42_encoding() {
        let input = "AC-42";
        let result = encode(input).unwrap();

        // Expected encoding according to spec:
        // Mode indicator: 001 (3 bits)
        // Character count: 00101 (5 = 5 bits)
        // First pair "AC": 10*45 + 12 = 462 -> 00111001110 (11 bits)
        // Second pair "-4": 41*45 + 4 = 1849 -> 11100111001 (11 bits)
        // Last char "2": 2 -> 000010 (6 bits)
        // Total data bits: 3 + 5 + 11 + 11 + 6 = 36 bits

        let mut cursor = std::io::Cursor::new(&result);
        let mut reader = BitReader::endian(&mut cursor, BigEndian);

        // Check mode and count
        let mode = reader.read::<u8>(3).unwrap();
        assert_eq!(mode, 0b001, "Mode indicator should be 001");

        let count = reader.read::<u8>(5).unwrap();
        assert_eq!(count, 5, "Character count should be 5");

        // Check first pair "AC"
        let ac_value = reader.read::<u16>(11).unwrap();
        assert_eq!(ac_value, 462, "AC should encode to 462 (10*45 + 12)");

        // Check second pair "-4"
        let dash4_value = reader.read::<u16>(11).unwrap();
        assert_eq!(dash4_value, 1849, "'-4' should encode to 1849 (41*45 + 4)");

        // Check last char "2"
        let two_value = reader.read::<u8>(6).unwrap();
        assert_eq!(two_value, 2, "2 should encode to 2");

        // Check terminator (should have room for full 9 bits)
        let terminator = reader.read::<u16>(9).unwrap();
        assert_eq!(terminator, 0, "Terminator should be all zeros");

        // Check padding bits
        let padding = reader.read::<u8>(3).unwrap();
        assert_eq!(padding, 0, "Padding bits should be all zeros");

        // Check first padding codeword
        let padding_codeword1 = reader.read::<u8>(8).unwrap();
        assert_eq!(
            padding_codeword1, 0xEC,
            "First padding codeword should be 11101100"
        );

        // Check second padding codeword
        let padding_codeword2 = reader.read::<u8>(8).unwrap();
        assert_eq!(
            padding_codeword2, 0x11,
            "Second padding codeword should be 00010001"
        );
    }

    #[test]
    fn test_partial_terminator() {
        // 20 chars = 3 + 5 + (10 * 11) = 118 bits
        // Leaves 10 bits: 9-bit terminator + 1 padding bit
        let result = encode("12345678901234567890").unwrap();

        let mut cursor = std::io::Cursor::new(&result);
        let mut reader = BitReader::endian(&mut cursor, BigEndian);

        // Skip the data bits
        reader.skip(118).unwrap();

        // Read the terminator (9 bits)
        let terminator = reader.read::<u16>(9).unwrap();
        assert_eq!(terminator, 0, "Terminator bits should be all zeros");

        // Read the padding bit
        let padding = reader.read::<u8>(1).unwrap();
        assert_eq!(padding, 0, "Padding bit should be zero");
    }

    #[test]
    fn test_truncated_terminator() {
        let input = "ABCDEFGHIJKLMNOPQRSTU"; // 21 chars
        let result = encode(input).unwrap();

        // Calculate bit usage:
        // Mode indicator:    3 bits
        // Character count:   5 bits
        // Full pairs: 10 × 11 bits = 110 bits
        // Last char:         6 bits
        // Total:           124 bits
        // Leaving:          4 bits for truncated terminator

        let mut cursor = std::io::Cursor::new(&result);
        let mut reader = BitReader::endian(&mut cursor, BigEndian);

        // Skip the data bits
        reader.skip(124).unwrap();

        // Read the truncated terminator (4 bits)
        let terminator = reader.read::<u8>(4).unwrap();
        assert_eq!(
            terminator, 0,
            "Truncated terminator bits should be all zeros"
        );
    }

    #[test]
    fn test_alphanumeric_char_mapping() {
        // Create test mappings according to Table 5
        let test_cases = [
            ('0', 0),
            ('1', 1),
            ('2', 2),
            ('3', 3),
            ('4', 4),
            ('5', 5),
            ('6', 6),
            ('7', 7),
            ('8', 8),
            ('9', 9),
            ('A', 10),
            ('B', 11),
            ('C', 12),
            ('D', 13),
            ('E', 14),
            ('F', 15),
            ('G', 16),
            ('H', 17),
            ('I', 18),
            ('J', 19),
            ('K', 20),
            ('L', 21),
            ('M', 22),
            ('N', 23),
            ('O', 24),
            ('P', 25),
            ('Q', 26),
            ('R', 27),
            ('S', 28),
            ('T', 29),
            ('U', 30),
            ('V', 31),
            ('W', 32),
            ('X', 33),
            ('Y', 34),
            ('Z', 35),
            (' ', 36),
            ('$', 37),
            ('%', 38),
            ('*', 39),
            ('+', 40),
            ('-', 41),
            ('.', 42),
            ('/', 43),
            (':', 44),
        ];

        // Test individual character values
        for (input_char, expected_value) in test_cases {
            let input = input_char.to_string();
            let result = encode(&input).unwrap();

            let mut cursor = std::io::Cursor::new(&result);
            let mut reader = BitReader::endian(&mut cursor, BigEndian);

            // Skip mode (3 bits) and count (5 bits)
            reader.skip(8).unwrap();

            // Read 6 bits for single character
            let value = reader.read::<u8>(6).unwrap();
            assert_eq!(
                value, expected_value,
                "Character '{}' should map to value {} but got {}",
                input_char, expected_value, value
            );
        }
    }

    #[test]
    fn test_alphanumeric_pair_mapping() {
        // Test pair encoding
        // According to spec: V = 45 × V1 + V2
        let test_pairs = [
            ("AB", 10 * 45 + 11), // A=10, B=11
            ("12", 1 * 45 + 2),   // 1=1, 2=2
            ("Z ", 35 * 45 + 36), // Z=35, space=36
            ("$%", 37 * 45 + 38), // $=37, %=38
            (":/", 44 * 45 + 43), // :=44, /=43
        ];

        for (pair, expected_value) in test_pairs {
            let result = encode(pair).unwrap();

            let mut cursor = std::io::Cursor::new(&result);
            let mut reader = BitReader::endian(&mut cursor, BigEndian);

            // Skip mode (3 bits) and count (5 bits)
            reader.skip(8).unwrap();

            // Read 11 bits for pair
            let value = reader.read::<u16>(11).unwrap();
            assert_eq!(
                value, expected_value,
                "Pair '{}' should encode to value {} but got {}",
                pair, expected_value, value
            );
        }
    }

    #[test]
    fn test_mixed_pair_and_char_encoding() {
        // Test cases with 3 characters (one pair + one char)
        let test_cases = [
            (
                "ABC", // A(10) B(11) paired = 461, C(12) single
                vec![
                    (11, 461), // pair AB: 10 * 45 + 11 = 461 (11 bits)
                    (6, 12),   // char C: 12 (6 bits)
                ],
            ),
            (
                "123", // 1(1) 2(2) paired = 47, 3(3) single
                vec![
                    (11, 47), // pair 12: 1 * 45 + 2 = 47
                    (6, 3),   // char 3: 3
                ],
            ),
            (
                "Z $", // Z(35) space(36) paired = 1611, $(37) single
                vec![
                    (11, 1611), // pair Z : 35 * 45 + 36 = 1611
                    (6, 37),    // char $: 37
                ],
            ),
        ];

        for (input, expected_values) in test_cases {
            let result = encode(input).unwrap();

            let mut cursor = std::io::Cursor::new(&result);
            let mut reader = BitReader::endian(&mut cursor, BigEndian);

            // Verify mode (should be 001 for alphanumeric in M4)
            let mode = reader.read::<u8>(3).unwrap();
            assert_eq!(mode, 0b001, "Mode indicator should be 001");

            // Verify count (should be 3)
            let count = reader.read::<u8>(5).unwrap();
            assert_eq!(count, 3, "Character count should be 3");

            // Verify each value
            for (bit_count, expected_value) in expected_values {
                let value = reader.read::<u16>(bit_count).unwrap();
                assert_eq!(
                    value, expected_value,
                    "For input '{}', expected value {} but got {}",
                    input, expected_value, value
                );
            }
        }
    }
}
