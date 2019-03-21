impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut num_counts = HashMap::new();
        let (short_nums, long_nums) = if nums1.len() > nums2.len() {
            (&nums2, &nums1)
        } else {
            (&nums1, &nums2)
        };
        for &num in short_nums {
            let count = num_counts.entry(num).or_insert(0);
            *count += 1;
        }
        let mut intersect = Vec::new();
        for num in long_nums {
            if let Some(count) = num_counts.get_mut(num) {
                if *count > 0 {
                    *count -= 1;
                    intersect.push(*num);
                }
            }
        }
        intersect
    }
}
