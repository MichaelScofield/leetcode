impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        if s.len() == 0 {
            return s;
        }
        let mut chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut j = chars.len() - 1;
        while i < j {
            if chars[i].is_ascii_alphabetic() {
                if chars[j].is_ascii_alphabetic() {
                    chars.swap(i, j);
                    i += 1;
                }
                j -= 1;
            } else {
                i += 1;
            }
        }
        chars.iter().collect()
    }
}
