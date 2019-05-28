impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let original = heights.clone();
        let heights = &mut { heights };
        heights.sort();
        let mut n = 0;
        for i in 0..original.len() {
            if original[i] != heights[i] {
                n += 1;
            }
        }
        n
    }
}
