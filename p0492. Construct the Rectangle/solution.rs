impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let sqrt = (area as f64).sqrt() as i32 + 1;
        let mut min_diff = std::i32::MAX;
        let mut rect = vec![];
        for i in (1..sqrt).rev() {
            if area % i == 0 {
                let j = area / i;
                let diff = (i - j).abs();
                if diff == 0 {
                    return vec![i, j];
                } else if diff < min_diff {
                    min_diff = diff;
                    rect = if i > j { vec![i, j] } else { vec![j, i] }
                }
            }
        }
        rect
    }
}
