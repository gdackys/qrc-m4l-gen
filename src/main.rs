mod alphanumeric_mode;
mod error_correction;
mod gf_256;

use gf_256::GF256;

const ERR_CORRECTION_CODEWORDS: usize = 8;

const GEN_COEFFS: [u8; 9] = [0x01, 0xff, 0x0b, 0x51, 0x36, 0xef, 0xad, 0xc8, 0x18];

fn main() {
    let data = "AC-42";

    let gf_256 = GF256::new();
    let encoded_data = alphanumeric_mode::encode(data).unwrap();
    let ec_codewords = ec_codewords(&encoded_data, &gf_256);

    println!("{:?}", encoded_data);
    println!("{:?}", ec_codewords);
}

fn ec_codewords(input: &[u8], gf_256: &GF256) -> Vec<u8> {
    error_correction::calculate_codewords(input, &GEN_COEFFS, gf_256, ERR_CORRECTION_CODEWORDS)
}
