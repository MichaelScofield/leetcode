impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut sum = 0;
        use std::collections::HashMap;
        let mut char_counts = HashMap::new();
        for c in chars.chars() {
            let count = char_counts.entry(c).or_insert(0);
            *count += 1;
        }
        for word in words {
            let mut i = 0;
            let mut is_good = true;
            for c in word.chars() {
                let count = char_counts.get_mut(&c);
                if let Some(count) = count {
                    if *count == 0 {
                        is_good = false;
                        break;
                    }
                    *count -= 1;
                    i += 1; // "borrow" from char_counts
                } else {
                    is_good = false;
                    break;
                }
            }
            if is_good {
                sum += word.len();
            }
            for c in word.chars() {
                if i == 0 {
                    break;
                }
                i -= 1; // *return* to char_counts
                let count = char_counts.get_mut(&c);
                *count.unwrap_or_else(|| panic!("algorithm has a bug!")) += 1;
            }
        }
        sum as i32
    }
}
