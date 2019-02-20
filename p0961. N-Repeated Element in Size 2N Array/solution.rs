impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut nums = HashSet::new();
        for x in a {
            if !nums.insert(x) {
                return x;
            }
        }
        panic!()
    }
}
