impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut current = 0;
        for num in nums {
            if num == 1 {
                current += 1;
            } else {
                if current > max {
                    max = current;
                }
                current = 0;
            }
        }
        std::cmp::max(max, current)
    }
}
