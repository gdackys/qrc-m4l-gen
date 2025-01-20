use crate::gf_256::GF256;

// error correction for M4-L symbols require 8 codewords
// const NUMBER_OF_CODEWORDS: usize = 5;

// const GEN_COEFFS: [u8; 6] = [0x01, 0x1f, 0xc6, 0x3f, 0x93, 0x74];

// const GEN_COEFFS: [u8; 9] = [0x01, 0xff, 0x0b, 0x51, 0x36, 0xef, 0xad, 0xc8, 0x18];

// static GF_256: GF256 = GF256::new();

pub fn calculate_error_correction(
    encoded_input: &[u8],
    coefficients: &[u8],
    galois_field: &GF256,
    number_of_codewords: usize,
) -> Vec<u8> {
    // Initialize registers
    let mut registers = vec![0; number_of_codewords];
    let length = registers.len();

    // Process each input codeword
    for input_byte in encoded_input {
        // XOR first register with next data byte
        let feedback_byte = (input_byte ^ registers[0]) as usize;

        // Update registers
        for i in 0..length - 1 {
            registers[i] = registers[i + 1]
                ^ galois_field.multiply(feedback_byte, coefficients[i + 1] as usize);
        }

        registers[length - 1] = galois_field.multiply(feedback_byte, coefficients[length] as usize);
    }

    registers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5_codewords() {
        let result = calculate_error_correction(
            // input fully encoded in numeric mode
            &[0b01000000, 0b00011000, 0b10101100, 0b11000011, 0b00000000],
            // precomputed generator coefficients for 5 codewords
            &[0x01, 0x1f, 0xc6, 0x3f, 0x93, 0x74],
            // Galois-field
            &GF256::new(),
            // number of codewords required
            5,
        );

        assert_eq!(result[0], 0b10000110);
        assert_eq!(result[1], 0b00001101);
        assert_eq!(result[2], 0b00100010);
        assert_eq!(result[3], 0b10101110);
        assert_eq!(result[4], 0b00110000);
    }

    #[test]
    fn test_10_codewords() {
        let result = calculate_error_correction(
            &[
                0b00010000, 0b00100000, 0b00001100, 0b01010110, 0b01100001, 0b10000000, 0b11101100,
                0b00010001, 0b11101100, 0b00010001, 0b11101100, 0b00010001, 0b11101100, 0b00010001,
                0b11101100, 0b00010001,
            ],
            &[
                0x01, 0xD8, 0xC2, 0x9F, 0x6F, 0xC7, 0x5E, 0x5F, 0x71, 0x9D, 0xC1,
            ],
            &GF256::new(),
            10,
        );

        assert_eq!(result[0], 0b10100101);
        assert_eq!(result[1], 0b00100100);
        assert_eq!(result[2], 0b11010100);
        assert_eq!(result[3], 0b11000001);
        assert_eq!(result[4], 0b11101101);
        assert_eq!(result[5], 0b00110110);
        assert_eq!(result[6], 0b11000111);
        assert_eq!(result[7], 0b10000111);
        assert_eq!(result[8], 0b00101100);
        assert_eq!(result[9], 0b01010101);
    }
}
