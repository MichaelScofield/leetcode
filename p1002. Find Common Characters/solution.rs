impl Solution {
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        let mut char_counts = HashMap::new();
        let word = &a[0];
        for c in word.chars() {
            let count = char_counts.entry(c).or_insert(0);
            *count += 1;
        }
        for i in 1..a.len() {
            let mut common_counts = HashMap::new();
            let word = &a[i];
            for c in word.chars() {
                let char_count = char_counts.get_mut(&c);
                if let Some(char_count) = char_count {
                    if *char_count > 0 {
                        *char_count -= 1;
                        let common_count = common_counts.entry(c).or_insert(0);
                        *common_count += 1;
                    }
                }
            }
            char_counts = common_counts;
        }
        let mut common_chars = Vec::new();
        for (ch, count) in char_counts.iter() {
            for _i in 0..*count {
                common_chars.push(ch.to_string());
            }
        }
        common_chars
    }
}
