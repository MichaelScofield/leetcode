impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        assert!(height.len() > 1);
        let mut max_area = 0;
        for step in 1..height.len() {
            for i in 0..height.len() - step {
                let area = std::cmp::min(height[i + step], height[i]) * step as i32;
                max_area = std::cmp::max(max_area, area);
            }
        }
        max_area
    }
}
