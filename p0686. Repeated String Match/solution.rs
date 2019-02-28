impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let chars_a: Vec<char> = a.chars().collect();
        let chars_b: Vec<char> = b.chars().collect();
        let len_a = chars_a.len();
        let len_b = chars_b.len();
        let mut p;
        'l: for i in 0..len_a {
            p = i;
            for j in 0..len_b {
                if chars_b[j] == chars_a[p % len_a] {
                    p += 1;
                } else {
                    continue 'l;
                }
            }
            return (p / len_a + if p % len_a > 0 { 1 } else { 0 }) as i32;
        }
        -1
    }
}
