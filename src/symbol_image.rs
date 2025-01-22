use crate::code_matrix::CodeMatrix;
use image::{ImageBuffer, Rgb};

const SYMBOL_SIZE: usize = 21;
const MODULE_COUNT: usize = 17;
const QUIET_ZONE: usize = 2;

pub fn write(file_name: &str, module_size: u32, matrix: &CodeMatrix) {
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(
        SYMBOL_SIZE as u32 * module_size,
        SYMBOL_SIZE as u32 * module_size,
    );

    img.fill(255);

    for i in 0..MODULE_COUNT {
        for j in 0..MODULE_COUNT {
            let module_value = matrix.read(i, j);
            put_module(&mut img, module_size, module_value, j, i);
        }
    }

    img.save(file_name).unwrap();
}

fn put_module(
    img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    module_size: u32,
    module_value: u8,
    x: usize,
    y: usize,
) {
    for i in 0..module_size {
        for j in 0..module_size {
            img.put_pixel(
                (QUIET_ZONE as u32 * module_size) + x as u32 * module_size + j,
                (QUIET_ZONE as u32 * module_size) + y as u32 * module_size + i,
                if module_value == 1 {
                    Rgb([0, 0, 0])
                } else {
                    Rgb([255, 255, 255])
                },
            )
        }
    }
}
