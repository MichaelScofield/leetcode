impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut new_key = Vec::<char>::new();
        let mut count = 0;
        for ch in s.chars().rev() {
            if ch != '-' {
                new_key.push(ch.to_ascii_uppercase());
                count += 1;
                if count % k == 0 {
                    new_key.push('-');
                }
            }
        }
        if new_key.len() > 0 && new_key[new_key.len() - 1] == '-' {
            new_key.pop();
        }
        new_key.iter().rev().collect::<String>()
    }
}
