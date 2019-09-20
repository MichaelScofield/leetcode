impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        use std::str::Chars;
        let mut commons = String::new();
        let mut strs: Vec<Chars> = strs.iter().map(|str| str.chars()).collect();
        'l: loop {
            let mut common = None;
            for str in strs.iter_mut() {
                if let Some(ch) = str.next() {
                    if let Some(c) = common {
                        if ch != c {
                            break 'l;
                        }
                    } else {
                        common = Some(ch);
                    }
                } else {
                    break 'l;
                }
            }
            if let Some(c) = common {
                commons.push(c);
            } else {
                break;
            }
        }
        commons
    }
}
