impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut absents = 0;
        let mut continuous_lates = 0;
        for c in s.chars() {
            if c == 'L' {
                if continuous_lates > 1 {
                    return false;
                }
                continuous_lates += 1;
            } else {
                if c == 'A' {
                    if absents > 0 {
                        return false;
                    }
                    absents += 1;
                }
                continuous_lates = 0;
            }
        }
        true
    }
}
