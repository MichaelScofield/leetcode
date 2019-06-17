impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => -1,
            1 => 0,
            _ => {
                let mut largest = -1;
                let mut index = -1;
                let mut second_largest = -1;
                for i in 0..nums.len() {
                    if nums[i] > largest {
                        second_largest = largest;
                        largest = nums[i];
                        index = i as i32;
                    } else if nums[i] > second_largest {
                        second_largest = nums[i];
                    }
                }
                if largest >= second_largest * 2 { index } else { -1 }
            }
        }
    }
}
