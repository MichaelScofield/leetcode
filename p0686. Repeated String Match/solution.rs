impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let mut s = String::new();
        let a = a.as_str();
        let b = b.as_str();
        let l = b.len();
        let mut repeated = 0;
        while s.len() < l {
            s.push_str(a);
            repeated += 1;
        }
        if s.contains(b) {
            repeated
        } else {
            s.push_str(a);
            if s.contains(b) { repeated + 1 } else { -1 }
        }
    }
}
