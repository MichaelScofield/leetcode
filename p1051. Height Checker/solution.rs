impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let original = heights.clone();
        let heights = &mut { heights };
        heights.sort();
        original.iter().zip(heights.iter()).filter(|(&i, &j)| i != j).count() as i32
    }
}
