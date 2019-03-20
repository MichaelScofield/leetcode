impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut positions = Vec::new();
        let mut prev = None;
        let mut start = 0;
        let mut end = 0;
        for ch in s.chars() {
            if let Some(c) = prev {
                if ch != c {
                    if end - start >= 3 {
                        positions.push(vec![start as i32, (end - 1) as i32]);
                    }
                    start = end;
                    prev = Some(ch);
                }
            } else {
                start = end;
                prev = Some(ch);
            }
            end += 1;
        }
        if end - start >= 3 {
            positions.push(vec![start as i32, (end - 1) as i32])
        }
        positions
    }
}
