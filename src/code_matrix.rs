use crate::bit_block::{BitBlock, DownwardsBlock, UpwardsBlock};
use crate::data_mask::DataMask;
use crate::pattern_scoring::PatternScoring;

pub struct CodeMatrix {
    data: [[u8; 17]; 17],
}

impl CodeMatrix {
    pub fn new() -> Self {
        Self {
            data: [[0; 17]; 17],
        }
    }

    fn init(&mut self) {
        self.data = [
            [1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ]
    }

    pub fn with_data_mask(&self, data_mask: &DataMask) -> Self {
        let mut matrix = self.clone();

        for i in 1..17 {
            for j in 1..17 {
                if i > 8 || j > 8 {
                    matrix.data[i][j] ^= data_mask.read(i, j);
                }
            }
        }

        matrix
    }

    fn clone(&self) -> Self {
        let mut instance = Self::new();

        for i in 0..17 {
            for j in 0..17 {
                instance.copy(i, j, self);
            }
        }

        instance
    }

    pub fn with_data(data: &[u8]) -> Self {
        let mut matrix = Self::new();

        matrix.init();
        matrix.write_data(data);

        matrix
    }

    fn write_data(&mut self, data: &[u8]) {
        self.write_block(&UpwardsBlock::new(data[0]), 13, 15);
        self.write_block(&UpwardsBlock::new(data[1]), 9, 15);
        self.write_block(&UpwardsBlock::new(data[2]), 5, 15);
        self.write_block(&UpwardsBlock::new(data[3]), 1, 15);

        self.write_block(&DownwardsBlock::new(data[4]), 1, 13);
        self.write_block(&DownwardsBlock::new(data[5]), 5, 13);
        self.write_block(&DownwardsBlock::new(data[6]), 9, 13);
        self.write_block(&DownwardsBlock::new(data[7]), 13, 13);

        self.write_block(&UpwardsBlock::new(data[8]), 13, 11);
        self.write_block(&UpwardsBlock::new(data[9]), 9, 11);
        self.write_block(&UpwardsBlock::new(data[10]), 5, 11);
        self.write_block(&UpwardsBlock::new(data[11]), 1, 11);

        self.write_block(&DownwardsBlock::new(data[12]), 1, 9);
        self.write_block(&DownwardsBlock::new(data[13]), 5, 9);
        self.write_block(&DownwardsBlock::new(data[14]), 9, 9);
        self.write_block(&DownwardsBlock::new(data[15]), 13, 9);

        self.write_block(&UpwardsBlock::new(data[16]), 13, 7);
        self.write_block(&UpwardsBlock::new(data[17]), 9, 7);

        self.write_block(&DownwardsBlock::new(data[18]), 9, 5);
        self.write_block(&DownwardsBlock::new(data[19]), 13, 5);

        self.write_block(&UpwardsBlock::new(data[20]), 13, 3);
        self.write_block(&UpwardsBlock::new(data[21]), 9, 3);

        self.write_block(&DownwardsBlock::new(data[22]), 9, 1);
        self.write_block(&DownwardsBlock::new(data[23]), 13, 1);
    }

    fn write_block(&mut self, block: &impl BitBlock, y: usize, x: usize) {
        for i in 0..4 {
            for j in 0..2 {
                self.write(y + i, x + j, block.read(i, j));
            }
        }
    }

    fn copy(&mut self, y: usize, x: usize, other: &Self) {
        self.write(y, x, other.read(y, x));
    }

    pub fn read(&self, y: usize, x: usize) -> u8 {
        self.data[y][x]
    }

    pub fn write(&mut self, y: usize, x: usize, value: u8) {
        self.data[y][x] = value;
    }

    pub fn with_format_info(&self, format_info: u16) -> Self {
        let mut matrix = self.clone();

        matrix.write(1, 8, ((format_info >> 0) & 1) as u8);
        matrix.write(2, 8, ((format_info >> 1) & 1) as u8);
        matrix.write(3, 8, ((format_info >> 2) & 1) as u8);
        matrix.write(4, 8, ((format_info >> 3) & 1) as u8);
        matrix.write(5, 8, ((format_info >> 4) & 1) as u8);
        matrix.write(6, 8, ((format_info >> 5) & 1) as u8);
        matrix.write(7, 8, ((format_info >> 6) & 1) as u8);
        matrix.write(8, 8, ((format_info >> 7) & 1) as u8);

        matrix.write(8, 7, ((format_info >> 8) & 1) as u8);
        matrix.write(8, 6, ((format_info >> 9) & 1) as u8);
        matrix.write(8, 5, ((format_info >> 10) & 1) as u8);
        matrix.write(8, 4, ((format_info >> 11) & 1) as u8);
        matrix.write(8, 3, ((format_info >> 12) & 1) as u8);
        matrix.write(8, 2, ((format_info >> 13) & 1) as u8);
        matrix.write(8, 1, ((format_info >> 14) & 1) as u8);

        matrix
    }
}

impl PatternScoring for CodeMatrix {
    fn count_right_edge(&self) -> u8 {
        self.data.iter().fold(0, |acc, &r| acc + r[16])
    }

    fn count_bottom_edge(&self) -> u8 {
        self.data[16].iter().fold(0, |acc, &v| acc + v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bit_block::UpwardsBlock;

    #[test]
    fn test_write_format_info_m4l() {
        let blank_matrix = CodeMatrix::new();

        // Write format information for M4-L
        let matrix = blank_matrix.with_format_info(0b101000010011001);

        // Test individual bits are written to correct positions
        assert_eq!(matrix.read(1, 8), 1); // Bit 0
        assert_eq!(matrix.read(2, 8), 0); // Bit 1
        assert_eq!(matrix.read(3, 8), 0); // Bit 2
        assert_eq!(matrix.read(4, 8), 1); // Bit 3
        assert_eq!(matrix.read(5, 8), 1); // Bit 4
        assert_eq!(matrix.read(6, 8), 0); // Bit 5
        assert_eq!(matrix.read(7, 8), 0); // Bit 6
        assert_eq!(matrix.read(8, 8), 1); // Bit 7
        assert_eq!(matrix.read(8, 7), 0); // Bit 8
        assert_eq!(matrix.read(8, 6), 0); // Bit 9
        assert_eq!(matrix.read(8, 5), 0); // Bit 10
        assert_eq!(matrix.read(8, 4), 0); // Bit 11
        assert_eq!(matrix.read(8, 3), 1); // Bit 12
        assert_eq!(matrix.read(8, 2), 0); // Bit 13
        assert_eq!(matrix.read(8, 1), 1); // Bit 14 (most significant)
    }

    #[test]
    fn test_with_data_mask() {
        let mut matrix = CodeMatrix::new();

        // Initialize some known data in the matrix
        matrix.init();

        // Set some test values in the region that should be masked (i > 8, j > 8)
        matrix.write(9, 9, 1);
        matrix.write(10, 10, 0);
        matrix.write(15, 15, 1);

        // Create a test mask pattern with known values
        let mut pattern = DataMask::new(9);

        // Set corresponding test values in the mask pattern
        pattern.write(9, 9, 1);
        pattern.write(10, 10, 1);
        pattern.write(15, 15, 0);

        // Apply the mask
        let masked_matrix = matrix.with_data_mask(&pattern);

        // Test that XOR was applied correctly in masked region (i > 8, j > 8)
        assert_eq!(masked_matrix.read(9, 9), 0); // 1 XOR 1 = 0
        assert_eq!(masked_matrix.read(10, 10), 1); // 0 XOR 1 = 1
        assert_eq!(masked_matrix.read(15, 15), 1); // 1 XOR 0 = 1

        // Test that data outside masked region remained unchanged
        assert_eq!(masked_matrix.read(0, 0), matrix.read(0, 0));
        assert_eq!(masked_matrix.read(7, 7), matrix.read(7, 7));
        assert_eq!(masked_matrix.read(8, 8), matrix.read(8, 8));
    }

    #[test]
    fn test_write_block() {
        let mut matrix = CodeMatrix::new();
        let upwards_block = UpwardsBlock::new(0b10101010);

        matrix.write_block(&upwards_block, 13, 15);

        assert_eq!(matrix.read(16, 16), 1);
        assert_eq!(matrix.read(16, 15), 0);
        assert_eq!(matrix.read(15, 16), 1);
        assert_eq!(matrix.read(15, 15), 0);
        assert_eq!(matrix.read(14, 16), 1);
        assert_eq!(matrix.read(14, 15), 0);
        assert_eq!(matrix.read(13, 16), 1);
        assert_eq!(matrix.read(13, 15), 0);
    }
}
