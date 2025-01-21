#[derive(Debug)]
pub struct MaskPattern {
    data: [[u8; 17]; 17],
}

impl MaskPattern {
    pub fn new() -> Self {
        MaskPattern {
            data: [[0; 17]; 17],
        }
    }

    pub fn pattern_00() -> Self {
        let mut pattern = Self::new();

        for i in 0..17 {
            for j in 0..17 {
                if i % 2 == 0 {
                    pattern.write(i, j, 1);
                }
            }
        }

        pattern
    }

    pub fn pattern_01() -> Self {
        let mut pattern = Self::new();

        for i in 0..17 {
            for j in 0..17 {
                if ((i / 2) + (j / 3)) % 2 == 0 {
                    pattern.write(i, j, 1);
                }
            }
        }

        pattern
    }

    pub fn pattern_10() -> Self {
        let mut pattern = Self::new();

        for i in 0..17 {
            for j in 0..17 {
                if ((i * j) % 2 + (i * j) % 3) % 2 == 0 {
                    pattern.write(i, j, 1);
                }
            }
        }

        pattern
    }

    pub fn pattern_11() -> Self {
        let mut pattern = Self::new();

        for i in 0..17 {
            for j in 0..17 {
                if ((i + j) % 2 + (i * j) % 3) % 2 == 0 {
                    pattern.write(i, j, 1);
                }
            }
        }

        pattern
    }

    pub fn read(&self, y: usize, x: usize) -> u8 {
        self.data[y][x]
    }

    pub fn write(&mut self, y: usize, x: usize, value: u8) {
        self.data[y][x] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl MaskPattern {
        pub fn get_row(&self, y: usize) -> &[u8] {
            &self.data[y]
        }
    }

    #[test]
    fn test_pattern_00() {
        let pattern = MaskPattern::pattern_00();

        let first_row = pattern.get_row(0);
        let second_row = pattern.get_row(1);
        let third_row = pattern.get_row(2);
        let second_last_row = pattern.get_row(15);
        let last_row = pattern.get_row(16);

        assert_eq!(first_row.iter().all(|&v| v == 1), true);
        assert_eq!(second_row.iter().all(|&v| v == 0), true);
        assert_eq!(third_row.iter().all(|&v| v == 1), true);
        assert_eq!(second_last_row.iter().all(|&v| v == 0), true);
        assert_eq!(last_row.iter().all(|&v| v == 1), true);
    }

    #[test]
    fn test_pattern_01() {
        let pattern = MaskPattern::pattern_01();

        let first_row = pattern.get_row(0);
        let second_row = pattern.get_row(1);
        let third_row = pattern.get_row(2);
        let fourth_row = pattern.get_row(3);
        let third_last_row = pattern.get_row(14);
        let second_last_row = pattern.get_row(15);
        let last_row = pattern.get_row(16);

        assert_eq!(
            first_row,
            [1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0]
        );
        assert_eq!(
            second_row,
            [1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0]
        );
        assert_eq!(
            third_row,
            [0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1]
        );
        assert_eq!(
            fourth_row,
            [0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1]
        );
        assert_eq!(
            third_last_row,
            [0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1]
        );
        assert_eq!(
            second_last_row,
            [0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1]
        );
        assert_eq!(
            last_row,
            [1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0]
        );
    }

    #[test]
    fn test_pattern_10() {
        let pattern = MaskPattern::pattern_10();

        let row_6 = pattern.get_row(6);
        let row_7 = pattern.get_row(7);
        let row_8 = pattern.get_row(8);
        let row_9 = pattern.get_row(9);
        let row_10 = pattern.get_row(10);
        let row_11 = pattern.get_row(11);
        let row_12 = pattern.get_row(12);

        assert_eq!(row_6, [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(row_7, [1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0]);
        assert_eq!(row_8, [1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1]);
        assert_eq!(row_9, [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]);
        assert_eq!(row_10, [1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0]);
        assert_eq!(row_11, [1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1]);
        assert_eq!(row_12, [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_pattern_11() {
        let pattern = MaskPattern::pattern_11();

        let row_11 = pattern.get_row(11);
        let row_12 = pattern.get_row(12);
        let row_13 = pattern.get_row(13);
        let row_14 = pattern.get_row(14);

        assert_eq!(row_11, [0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0]);
        assert_eq!(row_12, [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]);
        assert_eq!(row_13, [0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1]);
        assert_eq!(row_14, [1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1]);
    }
}
