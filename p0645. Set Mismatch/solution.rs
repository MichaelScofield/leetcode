impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let nums = &mut { nums };
        nums.sort();
        let mut dup = 0;
        for &num in nums.iter() {
            if dup == num {
                break;
            }
            dup = num;
        }
        let sum: i32 = nums.iter().sum();
        let correct_sum = ((1 + nums.len()) * nums.len() / 2) as i32;
        let error = correct_sum - sum;
        vec![dup, dup + error]
    }
}
