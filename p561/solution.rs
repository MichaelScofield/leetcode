impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums.clone();
        nums.sort();

        let mut sum = 0;
        for (i, x) in nums.iter().enumerate() {
            if i % 2 == 0 {
                sum += x;
            }
        }
        sum
    }
}
