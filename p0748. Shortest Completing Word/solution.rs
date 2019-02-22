impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        use std::collections::HashMap;
        let mut expect_chars = HashMap::<char, i32>::new();
        for c in license_plate.chars() {
            if c.is_ascii_alphabetic() {
                let entry = expect_chars.entry(c.to_ascii_lowercase()).or_default();
                *entry += 1;
            }
        }

        let mut shortest_completing_word = None;
        for word in words {
            let mut actual_chars = HashMap::<char, i32>::new();
            for c in word.chars() {
                if c.is_ascii_alphabetic() {
                    if expect_chars.get(&c) == None {
                        continue;
                    }
                    let entry = actual_chars.entry(c.to_ascii_lowercase()).or_default();
                    *entry += 1;
                }
            }
            let mut is_completing_word = true;
            for (c, n) in expect_chars.iter() {
                let v = actual_chars.get(c);
                if v == None || *v.unwrap() < *n {
                    is_completing_word = false;
                    break;
                }
            }
            if is_completing_word {
                if shortest_completing_word == None {
                    shortest_completing_word = Some(word);
                } else {
                    if word.len() < shortest_completing_word.as_ref().unwrap().len() {
                        shortest_completing_word = Some(word)
                    }
                }
            }
        }

        shortest_completing_word.unwrap()
    }
}
