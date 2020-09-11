impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let l = nums.len();
        if l == 0 {
            return vec![];
        }
        let mut ans = vec![0; l];
        let mut stack = vec![];
        for i in (0..l * 2).rev() {
            let num = nums[i % l];
            while let Some(&last) = stack.last() {
                if last <= num {
                    stack.pop();
                } else {
                    break;
                }
            }
            if i < l {
                ans[i] = if let Some(&last) = stack.last() {
                    last
                } else {
                    -1
                };
            }
            stack.push(num);
        }
        ans
    }
}
