impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut i = 0;
        let mut j = 1;
        let mut last_char = chars[0];
        let mut count = 1;
        while j < chars.len() {
            if chars[j] == last_char {
                count += 1;
            } else {
                chars[i] = last_char;
                i += 1;
                if count > 1 {
                    count.to_string().chars().for_each(|x| {
                        chars[i] = x;
                        i += 1;
                    });
                }
                last_char = chars[j];
                count = 1;
            }
            j += 1;
        }
        chars[i] = last_char;
        i += 1;
        if count > 1 {
            count.to_string().chars().for_each(|x| {
                chars[i] = x;
                i += 1;
            });
        }
        i as i32
    }
}
