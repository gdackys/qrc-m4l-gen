pub trait PatternScoring {
    fn count_right_edge(&self) -> u8;
    fn count_bottom_edge(&self) -> u8;

    fn pattern_score(&self) -> u16 {
        let right_edge = self.count_right_edge() as u16;
        let bottom_edge = self.count_bottom_edge() as u16;

        if right_edge <= bottom_edge {
            right_edge * 16 + bottom_edge
        } else {
            bottom_edge * 16 + right_edge
        }
    }
}
