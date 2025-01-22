mod alphanumeric_mode;
mod bit_block;
mod code_matrix;
mod data_mask;
mod error_correction;
mod format_info;
mod gf_256;
mod pattern_scoring;

use code_matrix::CodeMatrix;
use data_mask::DataMask;
use gf_256::GF256;

const ERR_CORRECTION_CODEWORDS: usize = 8;
const GEN_COEFFS: [u8; 9] = [0x01, 0xff, 0x0b, 0x51, 0x36, 0xef, 0xad, 0xc8, 0x18];

fn main() {
    // TODO: read the inputs from args
    let data = "HELLO WORLD";

    // TODO: validate inputs

    let gf_256 = GF256::new();
    let encoded_data = encode_data(data);
    let ec_codewords = gen_ec_codewords(&encoded_data, &gf_256);
    let data_codewords = combine_data(&encoded_data, &ec_codewords);

    let data_matrix = CodeMatrix::with_data(&data_codewords);
    let data_mask = DataMask::best_pattern(&data_matrix);
    let masked_matrix = data_matrix.with_data_mask(&data_mask);
    let format_info = get_format_info(&data_mask);
    let _final_matrix = masked_matrix.with_format_info(format_info);

    // TODO: write to an image file
}

fn encode_data(data: &str) -> Vec<u8> {
    alphanumeric_mode::encode(data).unwrap()
}

fn gen_ec_codewords(input: &[u8], gf_256: &GF256) -> Vec<u8> {
    error_correction::calculate_codewords(input, &GEN_COEFFS, gf_256, ERR_CORRECTION_CODEWORDS)
}

fn combine_data(encoded_data: &[u8], ec_codewords: &[u8]) -> Vec<u8> {
    [encoded_data, ec_codewords].concat()
}

fn get_format_info(data_mask: &DataMask) -> u16 {
    format_info::encode(data_mask.pattern_ref())
}
