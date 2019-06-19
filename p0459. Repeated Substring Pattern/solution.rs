impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let l = s.len();
        if l <= 1 {
            return false;
        }
        let chars: Vec<char> = s.chars().collect();
        let mut p = 1;
        'l: while p < l {
            while chars[p] != chars[0] {
                p += 1;
                if p == l {
                    return false;
                }
            }
            if l % p != 0 {
                p += 1;
                continue;
            }
            let d = l / p;
            for i in 0..p {
                for j in 0..d {
                    if chars[i + j * p] != chars[i] {
                        p += 1;
                        continue 'l;
                    }
                }
            }
            return true;
        }
        false
    }
}
