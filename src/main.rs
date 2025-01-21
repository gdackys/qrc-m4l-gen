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
use image::{ImageBuffer, Rgb};

const ERR_CORRECTION_CODEWORDS: usize = 8;
const GEN_COEFFS: [u8; 9] = [0x01, 0xff, 0x0b, 0x51, 0x36, 0xef, 0xad, 0xc8, 0x18];

fn main() {
    let data = "HELLO WORLD";

    let gf_256 = GF256::new();
    let encoded_data = encode_data(data);
    let ec_codewords = gen_ec_codewords(&encoded_data, &gf_256);
    let data_codewords = combine_data(&encoded_data, &ec_codewords);
    let data_matrix = CodeMatrix::with_data(&data_codewords);
    let data_mask = DataMask::best_pattern(&data_matrix);
    let data_mask_pattern = CodeMatrix::new().with_data_mask(&data_mask);
    let masked_matrix = data_matrix.with_data_mask(&data_mask);
    let format_info = get_format_info(&data_mask);
    let full_matrix = masked_matrix.with_format_info(format_info);

    save_image("data_mask_pattern.png", &data_mask_pattern);
    save_image("data_matrix.png", &data_matrix);
    save_image("masked_matrix.png", &masked_matrix);
    save_image("full_matrix.png", &full_matrix);
}

fn save_image(file_name: &str, matrix: &CodeMatrix) {
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(21, 21);

    img.fill(255);

    for i in 0..17 {
        for j in 0..17 {
            let pixel_value = matrix.read(i, j);

            img.put_pixel(
                (j as u32) + 2,
                (i as u32) + 2,
                if pixel_value == 1 {
                    Rgb([0, 0, 0])
                } else {
                    Rgb([255, 255, 255])
                },
            )
        }
    }

    img.save(file_name).unwrap();
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
