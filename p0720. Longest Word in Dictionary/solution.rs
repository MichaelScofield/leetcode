impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        use std::collections::HashMap;
        let mut words_length = HashMap::new();
        words.iter().for_each(|word|
            words_length.entry(word.len()).or_insert(vec![]).push(word));
        let mut candidates = words_length.get(&1usize).unwrap_or(&vec![]).clone();
        for len in 2usize.. {
            if let Some(words) = words_length.get(&len) {
                let mut new_candidates = Vec::new();
                for &word in words {
                    for &candidate in &candidates {
                        if word.starts_with(candidate) {
                            new_candidates.push(word);
                            break;
                        }
                    }
                }
                if new_candidates.is_empty() {
                    break;
                }
                candidates = new_candidates;
            } else {
                break;
            }
        }
        candidates.sort();
        if let Some(&word) = candidates.iter().next() { word.clone() } else { "".to_string() }
    }
}
