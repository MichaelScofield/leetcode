impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.len() == 0 {
            return true;
        }
        let chars = s.chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut j = chars.len() - 1;
        while i < j {
            if !chars[i].is_ascii_alphanumeric() {
                i += 1;
                continue;
            }
            if !chars[j].is_ascii_alphanumeric() {
                j -= 1;
                continue;
            }
            if chars[i].eq_ignore_ascii_case(&chars[j]) {
                i += 1;
                j -= 1;
            } else {
                return false;
            }
        }
        true
    }
}
