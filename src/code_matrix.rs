use crate::bit_block::{BitBlock, DownwardsBlock, UpwardsBlock};
use crate::mask_pattern::MaskPattern;

pub struct CodeMatrix {
    data: [[u8; 17]; 17],
}

impl CodeMatrix {
    fn new() -> Self {
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

    pub fn apply_mask(&self, mask_pattern: &MaskPattern) -> Self {
        let mut matrix = self.clone();

        for i in 0..17 {
            for j in 0..17 {
                if i > 8 && j > 8 {
                    matrix.data[i][j] ^= mask_pattern.read(i, j);
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
        matrix.place_data(data);

        matrix
    }

    fn place_data(&mut self, data: &[u8]) {
        self.place_block(&UpwardsBlock::new(data[0]), 13, 15);
        self.place_block(&UpwardsBlock::new(data[1]), 9, 15);
        self.place_block(&UpwardsBlock::new(data[2]), 5, 15);
        self.place_block(&UpwardsBlock::new(data[3]), 1, 15);

        self.place_block(&DownwardsBlock::new(data[4]), 1, 13);
        self.place_block(&DownwardsBlock::new(data[5]), 5, 13);
        self.place_block(&DownwardsBlock::new(data[6]), 9, 13);
        self.place_block(&DownwardsBlock::new(data[7]), 13, 13);

        self.place_block(&UpwardsBlock::new(data[8]), 13, 11);
        self.place_block(&UpwardsBlock::new(data[9]), 9, 11);
        self.place_block(&UpwardsBlock::new(data[10]), 5, 11);
        self.place_block(&UpwardsBlock::new(data[11]), 1, 11);

        self.place_block(&DownwardsBlock::new(data[12]), 1, 9);
        self.place_block(&DownwardsBlock::new(data[13]), 5, 9);
        self.place_block(&DownwardsBlock::new(data[14]), 9, 9);
        self.place_block(&DownwardsBlock::new(data[15]), 13, 9);

        self.place_block(&UpwardsBlock::new(data[16]), 13, 7);
        self.place_block(&UpwardsBlock::new(data[17]), 9, 7);

        self.place_block(&DownwardsBlock::new(data[18]), 9, 5);
        self.place_block(&DownwardsBlock::new(data[19]), 13, 5);

        self.place_block(&UpwardsBlock::new(data[20]), 13, 3);
        self.place_block(&UpwardsBlock::new(data[21]), 9, 3);

        self.place_block(&DownwardsBlock::new(data[22]), 9, 1);
        self.place_block(&DownwardsBlock::new(data[23]), 13, 1);
    }

    fn place_block(&mut self, block: &impl BitBlock, y: usize, x: usize) {
        for i in 0..4 {
            for j in 0..2 {
                self.write(y + i, x + j, block.read(i, j));
            }
        }
    }

    fn copy(&mut self, y: usize, x: usize, other: &Self) {
        self.write(y, x, other.read(y, x));
    }

    fn read(&self, y: usize, x: usize) -> u8 {
        self.data[y][x]
    }

    fn write(&mut self, y: usize, x: usize, value: u8) {
        self.data[y][x] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bit_block::UpwardsBlock;

    #[test]
    fn test_apply_mask() {
        let mut matrix = CodeMatrix::new();

        // Initialize some known data in the matrix
        matrix.init();

        // Set some test values in the region that should be masked (i > 8, j > 8)
        matrix.write(9, 9, 1);
        matrix.write(10, 10, 0);
        matrix.write(15, 15, 1);

        // Create a test mask pattern with known values
        let mut pattern = MaskPattern::new();

        // Set corresponding test values in the mask pattern
        pattern.write(9, 9, 1);
        pattern.write(10, 10, 1);
        pattern.write(15, 15, 0);

        // Apply the mask
        let masked_matrix = matrix.apply_mask(&pattern);

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
    fn test_place_block() {
        let mut matrix = CodeMatrix::new();
        let upwards_block = UpwardsBlock::new(0b10101010);

        matrix.place_block(&upwards_block, 13, 15);

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
