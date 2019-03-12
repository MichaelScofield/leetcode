impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        for i in 0..len {
            if i + 1 < len && nums[i] == nums[i + 1] {
                nums[i] = std::i32::MIN;
            }
        }
        let mut i = 0;
        let mut j = 0;
        while i < len {
            if nums[i] == std::i32::MIN {
                if j <= i {
                    j = i + 1;
                }
                while j < len {
                    if nums[j] > std::i32::MIN {
                        nums.swap(i, j);
                        break;
                    } else {
                        j += 1;
                    }
                }
                if j == len {
                    break;
                }
            }
            i += 1;
        }
        i as i32
    }
}
