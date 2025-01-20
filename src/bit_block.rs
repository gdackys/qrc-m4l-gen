pub trait BitBlock {
    fn value_at(&self, y: usize, x: usize) -> u8;
}

pub struct UpwardsBlock {
    data: [[u8; 2]; 4],
}

impl BitBlock for UpwardsBlock {
    fn value_at(&self, y: usize, x: usize) -> u8 {
        self.data[y][x]
    }
}

impl UpwardsBlock {
    pub fn new(input: u8) -> Self {
        Self {
            data: [
                [(input >> 0) & 1, (input >> 1) & 1],
                [(input >> 2) & 1, (input >> 3) & 1],
                [(input >> 4) & 1, (input >> 5) & 1],
                [(input >> 6) & 1, (input >> 7) & 1],
            ],
        }
    }
}

pub struct DownwardsBlock {
    data: [[u8; 2]; 4],
}

impl BitBlock for DownwardsBlock {
    fn value_at(&self, y: usize, x: usize) -> u8 {
        self.data[y][x]
    }
}

impl DownwardsBlock {
    pub fn new(input: u8) -> Self {
        Self {
            data: [
                [(input >> 6) & 1, (input >> 7) & 1],
                [(input >> 4) & 1, (input >> 5) & 1],
                [(input >> 2) & 1, (input >> 3) & 1],
                [(input >> 0) & 1, (input >> 1) & 1],
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upwards_block() {
        let block = UpwardsBlock::new(0b10101010);

        assert_eq!(block.value_at(3, 1), 1);
        assert_eq!(block.value_at(3, 0), 0);
        assert_eq!(block.value_at(2, 1), 1);
        assert_eq!(block.value_at(2, 0), 0);
        assert_eq!(block.value_at(1, 1), 1);
        assert_eq!(block.value_at(1, 0), 0);
        assert_eq!(block.value_at(0, 1), 1);
        assert_eq!(block.value_at(0, 0), 0);
    }

    #[test]
    fn test_downwards_block() {
        let block = DownwardsBlock::new(0b10101010);

        assert_eq!(block.value_at(0, 1), 1);
        assert_eq!(block.value_at(0, 0), 0);
        assert_eq!(block.value_at(1, 1), 1);
        assert_eq!(block.value_at(1, 0), 0);
        assert_eq!(block.value_at(2, 1), 1);
        assert_eq!(block.value_at(2, 0), 0);
        assert_eq!(block.value_at(3, 1), 1);
        assert_eq!(block.value_at(3, 0), 0);
    }
}
