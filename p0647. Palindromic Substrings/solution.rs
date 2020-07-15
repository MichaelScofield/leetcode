impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        fn is_palindromic(bytes: &[u8], i: usize, j: usize) -> bool {
            let mut i = i;
            let mut j = j - 1;
            while i < j {
                if bytes[i] != bytes[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }
            true
        }

        let bytes = s.as_bytes();
        let len = bytes.len();
        let mut count = 0;
        for step in 1..=len {
            for i in 0..len {
                let j = i + step;
                if j > len {
                    break;
                }
                count += if is_palindromic(bytes, i, j) {
                    1
                } else {
                    0
                }
            }
        }
        count
    }
}
