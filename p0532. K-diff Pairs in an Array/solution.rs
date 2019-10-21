impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        let nums = &mut { nums };
        nums.sort();
        let l = nums.len();
        let mut pairs = 0;
        for i in 0..l - 1 {
            if i > 0 {
                if nums[i] == nums[i - 1] {
                    continue;
                }
            }
            for j in i + 1..l {
                let diff = nums[j] - nums[i];
                if diff == k {
                    pairs += 1;
                    break;
                } else if diff > k {
                    break;
                }
            }
        }
        pairs
    }
}
