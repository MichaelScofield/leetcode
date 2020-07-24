impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();
        if len == 0 {
            return vec![-1, -1];
        }
        let mut i = 0;
        let mut j = len;
        let mut range = vec![-1, -1];
        while i < j {
            let mid = i + (j - i) / 2;
            if nums[mid] < target {
                i = mid + 1;
            } else if nums[mid] > target {
                j = mid;
            } else {
                let mut p = mid;
                while p > 0 {
                    if nums[p - 1] != target {
                        break;
                    }
                    p -= 1;
                }
                range[0] = p as i32;

                let mut q = mid;
                while q < len - 1 {
                    if nums[q + 1] != target {
                        break;
                    }
                    q += 1;
                }
                range[1] = q as i32;
                break;
            }
        }
        range
    }
}
