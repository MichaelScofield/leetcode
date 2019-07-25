impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut max_l = 1;
        let mut l = 1;
        for i in 0..nums.len() - 1 {
            if nums[i] < nums[i + 1] {
                l += 1;
            } else {
                if l > max_l {
                    max_l = l
                }
                l = 1;
            }
        }
        std::cmp::max(l, max_l)
    }
}
