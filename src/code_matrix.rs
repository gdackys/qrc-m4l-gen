use crate::bit_block::{BitBlock, DownwardsBlock, UpwardsBlock};

pub struct CodeMatrix {
    data: [[u8; 17]; 17],
}

pub fn with_data(data: &[u8]) -> CodeMatrix {
    let mut matrix = CodeMatrix::new();
    matrix.place_data(data);
    matrix
}

impl CodeMatrix {
    pub fn new() -> Self {
        Self {
            data: [
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
            ],
        }
    }

    pub fn place_data(&mut self, data: &[u8]) {
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
                self.data[y + i][x + j] = block.value_at(i, j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bit_block::UpwardsBlock;

    impl CodeMatrix {
        fn value_at(&self, y: usize, x: usize) -> u8 {
            self.data[y][x]
        }
    }

    #[test]
    fn test_place_block() {
        let mut matrix = CodeMatrix::new();
        let upwards_block = UpwardsBlock::new(0b10101010);

        matrix.place_block(&upwards_block, 13, 15);

        assert_eq!(matrix.value_at(16, 16), 1);
        assert_eq!(matrix.value_at(16, 15), 0);
        assert_eq!(matrix.value_at(15, 16), 1);
        assert_eq!(matrix.value_at(15, 15), 0);
        assert_eq!(matrix.value_at(14, 16), 1);
        assert_eq!(matrix.value_at(14, 15), 0);
        assert_eq!(matrix.value_at(13, 16), 1);
        assert_eq!(matrix.value_at(13, 15), 0);
    }
}
