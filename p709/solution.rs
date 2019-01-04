impl Solution {
    pub fn to_lower_case(str: String) -> String {
        let mut s = String::new();
        for c in str.chars() {
            s.push_str(c.to_lowercase().to_string().as_str())
        }
        return s;
    }
}
