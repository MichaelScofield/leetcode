impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::HashMap;
        let mut mappings = HashMap::with_capacity(26);
        let mut chars_s = s.chars();
        let mut chars_t = t.chars();
        loop {
            let char_s = chars_s.next();
            let char_t = chars_t.next();
            if char_s == None { // s and t have same length
                break;
            }
            let char_s = char_s.unwrap();
            let char_t = char_t.unwrap();
            let mapping = mappings.entry(char_s).or_insert(char_t);
            if *mapping != char_t {
                return false;
            }
        }
        use std::collections::HashSet;
        let mut set = HashSet::with_capacity(26);
        for v in mappings.values() {
            if !set.insert(v) {
                return false;
            }
        }
        true
    }
}
