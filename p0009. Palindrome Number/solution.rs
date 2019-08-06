impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let string = x.to_string();
        let rev_string: String = string.chars().rev().collect();
        string == rev_string
    }
}
