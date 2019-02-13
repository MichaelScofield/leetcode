impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        use std::collections::HashMap;
        let mut orders = HashMap::with_capacity(order.len());
        for (i, c) in order.chars().enumerate() {
            orders.insert(c, i);
        }
        for i in 0..words.len() - 1 {
            let mut word1 = words[i].chars();
            let mut word2 = words[i + 1].chars();
            loop {
                let char1 = word1.next();
                let char2 = word2.next();
                if char1 == None {
                    break;
                } else {
                    if char2 == None {
                        return false;
                    } else {
                        let char1 = char1.unwrap();
                        let char2 = char2.unwrap();
                        let order1 = *orders.get(&char1).unwrap();
                        let order2 = *orders.get(&char2).unwrap();
                        if order1 > order2 {
                            return false;
                        } else if order1 < order2 {
                            break;
                        }
                    }
                }
            }
        }
        true
    }
}
