impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        use std::collections::HashMap;
        let mut chars = HashMap::new();
        for c in s.chars() {
            let entry = chars.entry(c).or_insert(0);
            *entry += 1;
        }
        if chars.len() == 1 {
            return s.len() as i32;
        }

        let mut longest_palindrome = 0;
        let mut can_put_single_char_in_middle = false;
        for (_c, n) in chars {
            if n % 2 == 0 {
                longest_palindrome += n;
            } else {
                longest_palindrome += n - 1;
                can_put_single_char_in_middle = true;
            }
        }
        if can_put_single_char_in_middle {
            longest_palindrome + 1
        } else {
            longest_palindrome
        }
    }
}
