impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let indices = map.entry(num).or_insert(vec![]);
            if let Some(last) = indices.last() {
                if i - *last <= k as usize {
                    return true;
                }
            }
            indices.push(i);
        }
        false
    }
}
