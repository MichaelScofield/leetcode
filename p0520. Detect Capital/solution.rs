impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        if word.len() == 1 {
            return true;
        }
        let mut chars = word.chars();
        let is_first_letter_capital = chars.next().unwrap().is_ascii_uppercase();
        if is_first_letter_capital {
            let is_second_letter_capital = chars.next().unwrap().is_ascii_uppercase();
            if is_second_letter_capital {
                for c in chars {
                    if !c.is_ascii_uppercase() {
                        return false;
                    }
                }
            } else {
                for c in chars {
                    if c.is_ascii_uppercase() {
                        return false;
                    }
                }
            }
        } else {
            for c in chars {
                if c.is_ascii_uppercase() {
                    return false;
                }
            }
        }
        true
    }
}
