impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 | 2 => -1,
            1 => 0,
            _ => {
                let mut left_sum = 0;
                let mut right_sum = nums.iter().sum::<i32>();
                for i in 0..nums.len() {
                    right_sum -= nums[i];
                    if left_sum == right_sum {
                        return i as i32;
                    }
                    left_sum += nums[i];
                }
                -1
            }
        }
    }
}
