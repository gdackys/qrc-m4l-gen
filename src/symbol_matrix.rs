use crate::code_matrix::CodeMatrix;
use crate::data_mask::DataMask;
use crate::format_info;

pub fn generate(data_codewords: &[u8]) {
    let data_matrix = CodeMatrix::with_data(&data_codewords);
    let data_mask = DataMask::best_pattern(&data_matrix);

    let masked_matrix = data_matrix.with_data_mask(&data_mask);
    let format_info = format_info::encode(data_mask.pattern_ref());

    masked_matrix.with_format_info(format_info);
}
