impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        if l == 0 {
            return 0;
        }
        let mut min_num_in_subarray = std::i32::MAX;
        let mut max_num_in_subarray = std::i32::MIN;
        for i in 0..l - 1 {
            if nums[i] > nums[i + 1] {
                if nums[i + 1] < min_num_in_subarray {
                    min_num_in_subarray = nums[i + 1];
                }
                if nums[i] > max_num_in_subarray {
                    max_num_in_subarray = nums[i];
                }
            }
        }
        if min_num_in_subarray == std::i32::MAX && max_num_in_subarray == std::i32::MIN {
            return 0;
        }
        let mut insert_min_num_pos = 0;
        let mut insert_max_num_pos = l - 1;
        for i in 0..l {
            if nums[i] <= min_num_in_subarray {
                insert_min_num_pos = i + 1;
            } else {
                break;
            }
        }
        for i in (0..l).rev() {
            if nums[i] >= max_num_in_subarray {
                insert_max_num_pos = i - 1;
            } else {
                break;
            }
        }
        (insert_max_num_pos - insert_min_num_pos + 1) as i32
    }
}
