impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let nums = &mut { nums };
        nums.sort();
        let mut i = 0;
        let mut j = nums.len() as i32 - 1;
        while i <= j {
            let mid = (j - i) / 2 + i;
            if nums[mid as usize] == mid {
                i = mid + 1;
            } else {
                j = mid - 1;
            }
        }
        i
    }
}
