impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        if haystack.len() < needle.len() {
            return -1;
        }
        let haystack_chars: Vec<char> = haystack.chars().collect();
        let needle_chars: Vec<char> = needle.chars().collect();
        for p in 0..haystack_chars.len() - needle_chars.len() + 1 {
            let mut i = p;
            let mut j = 0;
            while haystack_chars[i] == needle_chars[j] {
                i += 1;
                j += 1;
                if j == needle_chars.len() {
                    return p as i32;
                }
            }
        }
        -1
    }
}
