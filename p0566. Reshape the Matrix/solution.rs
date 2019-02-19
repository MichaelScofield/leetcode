impl Solution {
    pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let r = r as usize;
        let c = c as usize;
        let n = nums.len();
        let m = nums[0].len();
        if r * c != m * n {
            return nums;
        }

        let mut reshaped = Vec::new();
        let mut i = 0;
        let mut j = 0;
        for _ in 0..r {
            let mut row = Vec::with_capacity(c);
            for _ in 0..c {
                row.push(nums[j][i]);
                i += 1;
                if i >= m {
                    i = 0;
                    j += 1;
                }
            }
            reshaped.push(row);
        }
        reshaped
    }
}
