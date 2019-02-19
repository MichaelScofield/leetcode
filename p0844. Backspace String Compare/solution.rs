impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        Self::get_backspaced(s) == Self::get_backspaced(t)
    }

    fn get_backspaced(s: String) -> Vec<char> {
        let mut chars = Vec::with_capacity(s.len());
        for c in s.chars() {
            if c == '#' {
                chars.pop();
            } else {
                chars.push(c);
            }
        }
        chars
    }
}
