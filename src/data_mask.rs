use crate::code_matrix::CodeMatrix;
use crate::pattern_scoring::PatternScoring;

pub struct DataMask {
    data: [[u8; 17]; 17],
    pattern_reference: u8,
}

impl DataMask {
    pub fn pattern_ref(&self) -> u8 {
        self.pattern_reference
    }

    pub fn best_pattern(matrix: &CodeMatrix) -> DataMask {
        let patterns = [
            Self::pattern_00(),
            Self::pattern_01(),
            Self::pattern_10(),
            Self::pattern_11(),
        ];

        patterns
            .into_iter()
            .max_by_key(|pattern| matrix.with_data_mask(pattern).pattern_score())
            .unwrap()
    }

    pub fn new(pattern_reference: u8) -> Self {
        DataMask {
            data: [[0; 17]; 17],
            pattern_reference,
        }
    }

    pub fn pattern_00() -> Self {
        let mut mask = Self::new(0);

        for i in 0..17 {
            for j in 0..17 {
                if i % 2 == 0 {
                    mask.write(i, j, 1);
                }
            }
        }

        mask
    }

    pub fn pattern_01() -> Self {
        let mut mask = Self::new(1);

        for i in 0..17 {
            for j in 0..17 {
                if ((i / 2) + (j / 3)) % 2 == 0 {
                    mask.write(i, j, 1);
                }
            }
        }

        mask
    }

    pub fn pattern_10() -> Self {
        let mut mask = Self::new(2);

        for i in 0..17 {
            for j in 0..17 {
                if ((i * j) % 2 + (i * j) % 3) % 2 == 0 {
                    mask.write(i, j, 1);
                }
            }
        }

        mask
    }

    pub fn pattern_11() -> Self {
        let mut mask = Self::new(3);

        for i in 0..17 {
            for j in 0..17 {
                if ((i + j) % 2 + (i * j) % 3) % 2 == 0 {
                    mask.write(i, j, 1);
                }
            }
        }

        mask
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

    impl DataMask {
        pub fn read_row(&self, y: usize) -> &[u8] {
            &self.data[y]
        }
    }

    // Helper function to create a test matrix with known values
    fn create_test_matrix() -> CodeMatrix {
        let mut matrix = CodeMatrix::new();

        // Initialize with all zeros first
        for i in 0..17 {
            for j in 0..17 {
                matrix.write(i, j, 0);
            }
        }

        // Set some test values in regions that will be masked (i > 8, j > 8)
        matrix.write(9, 9, 1);
        matrix.write(10, 10, 1);
        matrix.write(15, 15, 0);
        matrix.write(16, 16, 1);

        // Set some values in the right edge to influence scoring
        matrix.write(0, 16, 1);
        matrix.write(2, 16, 1);
        matrix.write(4, 16, 1);

        // Set some values in the bottom edge to influence scoring
        matrix.write(16, 0, 1);
        matrix.write(16, 2, 1);
        matrix.write(16, 4, 1);

        matrix
    }

    #[test]
    fn test_best_pattern_returns_best_pattern() {
        let matrix = create_test_matrix();
        let best_pattern = DataMask::best_pattern(&matrix);

        // Apply each pattern and get scores
        let scores = vec![
            matrix
                .with_data_mask(&DataMask::pattern_00())
                .pattern_score(),
            matrix
                .with_data_mask(&DataMask::pattern_01())
                .pattern_score(),
            matrix
                .with_data_mask(&DataMask::pattern_10())
                .pattern_score(),
            matrix
                .with_data_mask(&DataMask::pattern_11())
                .pattern_score(),
        ];

        // Get the maximum score
        let max_score = scores.iter().max().unwrap();

        // Apply the selected best pattern
        let result_score = matrix.with_data_mask(&best_pattern).pattern_score();

        // The selected pattern should produce the highest score
        assert_eq!(result_score, *max_score);
    }

    #[test]
    fn test_best_pattern_different_inputs() {
        let matrix1 = create_test_matrix();
        let mut matrix2 = create_test_matrix();

        // Modify matrix2 to be different from matrix1
        matrix2.write(9, 9, 0);
        matrix2.write(10, 10, 0);
        matrix2.write(15, 15, 1);

        let pattern1 = DataMask::best_pattern(&matrix1);
        let pattern2 = DataMask::best_pattern(&matrix2);

        // Different input matrices should potentially result in different optimal patterns
        // Compare pattern scores rather than the patterns themselves
        let score1 = matrix1.with_data_mask(&pattern1).pattern_score();
        let score2 = matrix2.with_data_mask(&pattern2).pattern_score();

        // Each pattern should be optimal for its own matrix
        let alternate_score1 = matrix1.with_data_mask(&pattern2).pattern_score();
        let alternate_score2 = matrix2.with_data_mask(&pattern1).pattern_score();

        assert!(score1 >= alternate_score1);
        assert!(score2 >= alternate_score2);
    }

    #[test]
    fn test_best_pattern_edge_cases() {
        let mut matrix = CodeMatrix::new();

        // Test with all zeros
        for i in 0..17 {
            for j in 0..17 {
                matrix.write(i, j, 0);
            }
        }
        let pattern1 = DataMask::best_pattern(&matrix);

        // Test with all ones in maskable region
        for i in 9..17 {
            for j in 9..17 {
                matrix.write(i, j, 1);
            }
        }
        let pattern2 = DataMask::best_pattern(&matrix);

        // Verify that patterns are different for different inputs
        let mut patterns_differ = false;
        for i in 0..17 {
            for j in 0..17 {
                if pattern1.read(i, j) != pattern2.read(i, j) {
                    patterns_differ = true;
                    break;
                }
            }
        }

        assert!(patterns_differ);
    }

    #[test]
    fn test_pattern_00() {
        let pattern = DataMask::pattern_00();

        let first_row = pattern.read_row(0);
        let second_row = pattern.read_row(1);
        let third_row = pattern.read_row(2);
        let second_last_row = pattern.read_row(15);
        let last_row = pattern.read_row(16);

        assert_eq!(first_row.iter().all(|&v| v == 1), true);
        assert_eq!(second_row.iter().all(|&v| v == 0), true);
        assert_eq!(third_row.iter().all(|&v| v == 1), true);
        assert_eq!(second_last_row.iter().all(|&v| v == 0), true);
        assert_eq!(last_row.iter().all(|&v| v == 1), true);
    }

    #[test]
    fn test_pattern_01() {
        let pattern = DataMask::pattern_01();

        let first_row = pattern.read_row(0);
        let second_row = pattern.read_row(1);
        let third_row = pattern.read_row(2);
        let fourth_row = pattern.read_row(3);
        let third_last_row = pattern.read_row(14);
        let second_last_row = pattern.read_row(15);
        let last_row = pattern.read_row(16);

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
        let pattern = DataMask::pattern_10();

        let row_6 = pattern.read_row(6);
        let row_7 = pattern.read_row(7);
        let row_8 = pattern.read_row(8);
        let row_9 = pattern.read_row(9);
        let row_10 = pattern.read_row(10);
        let row_11 = pattern.read_row(11);
        let row_12 = pattern.read_row(12);

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
        let pattern = DataMask::pattern_11();

        let row_11 = pattern.read_row(11);
        let row_12 = pattern.read_row(12);
        let row_13 = pattern.read_row(13);
        let row_14 = pattern.read_row(14);

        assert_eq!(row_11, [0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0]);
        assert_eq!(row_12, [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]);
        assert_eq!(row_13, [0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1]);
        assert_eq!(row_14, [1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1]);
    }
}
