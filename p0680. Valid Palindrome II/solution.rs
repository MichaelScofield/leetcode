impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        if s.len() == 0 {
            return true;
        }

        fn is_palindrome(chars: &Vec<char>, i: usize, j: usize) -> bool {
            let mut i = i;
            let mut j = j;
            while i < j {
                if chars[i] == chars[j] {
                    i += 1;
                    j -= 1;
                } else {
                    return false;
                }
            }
            true
        }

        let chars = s.chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut j = chars.len() - 1;
        while i < j {
            if chars[i] == chars[j] {
                i += 1;
                j -= 1;
            } else {
                return is_palindrome(&chars, i + 1, j) || is_palindrome(&chars, i, j - 1);
            }
        }
        true
    }
}
