impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        fn longest_palindrome(bytes: &[u8], i: usize, j: usize) -> (usize, usize) {
            let mut i = i;
            let mut j = j;
            let mut r = (i, j);
            let l = bytes.len() - 1;
            loop {
                if i == 0 || j == l {
                    break;
                }
                if bytes[i - 1] != bytes[j + 1] {
                    break;
                }
                i -= 1;
                j += 1;
                r = (i, j);
            }
            r
        }

        if s.len() == 0 {
            return "".to_string();
        }
        let bytes = s.as_bytes();
        let l = bytes.len();
        let mut longest = (0, 0);
        for i in 0..l {
            let r = longest_palindrome(bytes, i, i);
            if r.1 - r.0 > longest.1 - longest.0 {
                longest = r
            }
            if i + 1 < l && bytes[i] == bytes[i + 1] {
                let r = longest_palindrome(bytes, i, i + 1);
                if r.1 - r.0 > longest.1 - longest.0 {
                    longest = r
                }
            }
        }
        std::str::from_utf8(&bytes[longest.0..=longest.1]).unwrap().to_string()
    }
}
