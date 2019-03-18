impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut sum: i64 = 0;
        let mut min: i32 = std::i32::MAX;
        let l = nums.len();
        for num in nums {
            sum += num as i64;
            if num < min {
                min = num;
            }
        }
        (sum - l as i64 * min as i64) as i32
    }
}
