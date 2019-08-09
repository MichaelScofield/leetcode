impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        if s.len() == 0 {
            return s;
        }
        fn is_vowel(c: char) -> bool {
            c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
        }
        let mut i = 0;
        let mut j = s.len() - 1;
        let mut chars = s.chars().collect::<Vec<char>>();
        while i < j {
            if !is_vowel(chars[i].to_ascii_lowercase()) {
                i += 1;
            } else {
                if !is_vowel(chars[j].to_ascii_lowercase()) {
                    j -= 1;
                } else {
                    chars.swap(i, j);
                    i += 1;
                    j -= 1;
                }
            }
        }
        chars.iter().collect()
    }
}
