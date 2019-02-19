impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut reversed = String::new();
        let words = s.split_whitespace().collect::<Vec<&str>>();
        let l = words.len();
        for (i, word) in words.iter().enumerate() {
            reversed.push_str(&word.chars().rev().collect::<String>());
            if i < l - 1 {
                reversed.push_str(&" ");
            }
        }
        reversed
    }
}
