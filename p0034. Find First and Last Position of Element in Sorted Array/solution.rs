impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }
        fn bound_binary_search(nums: &Vec<i32>, target: i32, left: bool) -> i32 {
            let mut i = 0 as i32;
            let mut j = (nums.len() - 1) as i32;
            while i <= j {
                let mid = (j - i) / 2 + i;
                if nums[mid as usize] == target {
                    if left {
                        j = mid - 1;
                    } else {
                        i = mid + 1;
                    }
                } else if nums[mid as usize] > target {
                    j = mid - 1;
                } else {
                    i = mid + 1;
                }
            }
            if left { j + 1 } else { i - 1 }
        }
        let i = bound_binary_search(&nums, target, true);
        if i == -1 || i == nums.len() as i32 || nums[i as usize] != target {
            return vec![-1, -1];
        }
        let j = bound_binary_search(&nums, target, false);
        vec![i, j]
    }
}
