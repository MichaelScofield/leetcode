impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let n = nums.len();
        let mut dp = vec![0; n];
        let mut max = std::i32::MIN;
        for i in 1..=n {
            for j in 0..=n - i {
                dp[j] = dp[j] + nums[j + i - 1];
                max = std::cmp::max(max, dp[j]);
            }
        }
        max
    }
}
