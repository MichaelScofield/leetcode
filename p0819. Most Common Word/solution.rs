impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        use std::collections::HashMap;
        let mut words = HashMap::new();
        let chars = paragraph.chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut has_word = false;
        for (j, c) in chars.iter().enumerate() {
            if !c.is_ascii_alphabetic() {
                if has_word {
                    let word = chars[i..j].iter().collect::<String>().to_lowercase();
                    let count = words.entry(word).or_insert(0);
                    *count += 1;
                    has_word = false;
                }
            } else {
                if !has_word {
                    i = j;
                    has_word = true;
                }
            }
        }
        if has_word {
            let word = chars[i..].iter().collect::<String>().to_lowercase();
            let count = words.entry(word).or_insert(0);
            *count += 1;
        }
        let mut most_common_word = "".to_string();
        let mut max = 0;
        for (word, count) in words {
            if count > max && !banned.contains(&word) {
                most_common_word = word;
                max = count;
            }
        }
        most_common_word
    }
}
