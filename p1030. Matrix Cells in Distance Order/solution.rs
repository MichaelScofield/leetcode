impl Solution {
    pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        use std::cmp::max;
        use std::collections::HashSet;
        let mut result = Vec::new();
        let mut cells = HashSet::new();
        for d in 0..(max(r - r0, r0) + max(c - c0, c0) + 1) {
            cells.clear();
            for i in 0..(d + 1) {
                let j = d - i;
                if r0 + i < r {
                    if c0 + j < c {
                        cells.insert((r0 + i, c0 + j));
                    }
                    if c0 - j >= 0 {
                        cells.insert((r0 + i, c0 - j));
                    }
                }
                if r0 - i >= 0 {
                    if c0 + j < c {
                        cells.insert((r0 - i, c0 + j));
                    }
                    if c0 - j >= 0 {
                        cells.insert((r0 - i, c0 - j));
                    }
                }
            }
            for &(r, c) in cells.iter() {
                result.push(vec![r, c]);
            }
        }
        result
    }
}
