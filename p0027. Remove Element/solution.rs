impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }
        let mut i = 0;
        let mut j = len - 1;
        while i < j {
            if nums[i] == val {
                nums.swap(i, j);
                j -= 1;
            } else {
                i += 1;
            }
        }
        if nums[i] == val {
            i as i32
        } else {
            (i + 1) as i32
        }
    }
}
