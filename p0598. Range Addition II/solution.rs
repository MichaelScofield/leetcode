impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        if ops.len() < 1 {
            return m * n;
        }
        let mut min_a = std::i32::MAX;
        let mut min_b = std::i32::MAX;
        for op in ops {
            if op[0] < min_a {
                min_a = op[0];
            }
            if op[1] < min_b {
                min_b = op[1];
            }
        }
        min_a * min_b
    }
}
