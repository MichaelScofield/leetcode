impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        use std::collections::HashMap;
        let mut letters = HashMap::new();
        for c in magazine.chars() {
            let count = letters.entry(c).or_insert(0);
            *count += 1;
        }
        for c in ransom_note.chars() {
            if let Some(count) = letters.get_mut(&c) {
                *count -= 1;
                if *count < 0 {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}
