impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() <= 1 {
            return;
        }
        let mut i = 0;
        let mut j = nums.len() - 1;
        let mut p = 0;
        while p <= j {
            if nums[p] == 0 {
                nums.swap(i, p);
                i += 1;
                p += 1;
            } else if nums[p] == 2 {
                nums.swap(j, p);
                if j == 0 {
                    return;
                } else {
                    j -= 1;
                }
            } else {
                p += 1;
            }
        }
    }
}
