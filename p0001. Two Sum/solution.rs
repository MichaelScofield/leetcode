impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let l = nums.len();
        for i in 0..l {
            let residue = target - nums[i];
            for j in i + 1..l {
                if nums[j] == residue {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}
