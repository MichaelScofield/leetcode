impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        match nums.len() {
            0 => -1,
            _ => {
                let mut i = 0;
                let mut j = nums.len();
                while i < j {
                    let mid = i + (j - i) / 2;
                    if nums[mid] == target {
                        return mid as i32;
                    } else if nums[mid] < target {
                        i = mid + 1;
                    } else {
                        j = mid;
                    }
                }
                -1
            }
        }
    }
}
