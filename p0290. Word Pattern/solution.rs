impl Solution {
    pub fn word_pattern(pattern: String, str: String) -> bool {
        let mut mappings = vec![None; 26];
        let mut letters = pattern.chars();
        let mut words = str.split_whitespace();
        loop {
            let letter = letters.next();
            let word = words.next();
            if letter == None && word == None {
                break;
            }
            if letter != None && word != None {
                let letter: usize = letter.unwrap() as usize;
                let word = word.unwrap();
                if let Some(mapping) = mappings[letter - 97] {
                    if mapping != word {
                        return false;
                    }
                } else {
                    mappings[letter - 97] = Some(word);
                }
            } else {
                return false;
            }
        }
        use std::collections::HashSet;
        let mut set = HashSet::with_capacity(26);
        for v in mappings {
            if let Some(mapping) = v {
                if !set.insert(mapping) {
                    return false;
                }
            }
        }
        true
    }
}
