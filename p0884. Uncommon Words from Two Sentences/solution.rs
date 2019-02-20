use std::collections::HashMap;

impl Solution {
    pub fn uncommon_from_sentences(a: String, b: String) -> Vec<String> {
        let words_a = Self::get_unique_words(&a);
        let words_b = Self::get_unique_words(&b);
        let mut uncommon_words = Vec::new();
        Self::get_uncommon_words(&mut uncommon_words, &words_a, &words_b);
        Self::get_uncommon_words(&mut uncommon_words, &words_b, &words_a);
        uncommon_words
    }

    fn get_unique_words(s: &String) -> HashMap<&str, u32> {
        let mut words: HashMap<&str, u32> = HashMap::new();
        for word in s.split_whitespace() {
            let v = words.entry(word).or_insert(0);
            *v += 1;
        }
        words
    }

    fn get_uncommon_words<'a>(uncommon_words: &mut Vec<String>,
                              words1: &'a HashMap<&str, u32>, words2: &'a HashMap<&str, u32>) {
        for (s, c) in words1 {
            if *c == 1 && !words2.contains_key(s) {
                uncommon_words.push(s.to_string())
            }
        }
    }
}
