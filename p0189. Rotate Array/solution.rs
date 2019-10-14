impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let l = nums.len();
        for _n in 0..k {
            let mut last = nums[0];
            for i in 0..l {
                let j = (i + 1) % l;
                let tmp = nums[j];
                nums[j] = last;
                last = tmp;
            }
        }
    }
}
