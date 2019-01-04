impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        use ::std::collections::HashSet;

        let mut jewel_chars = HashSet::new();
        for c in j.chars() {
            jewel_chars.insert(c);
        }

        let mut i = 0;
        for c in s.chars() {
            if jewel_chars.contains(&c) {
                i += 1;
            }
        }
        return i;
    }
}
