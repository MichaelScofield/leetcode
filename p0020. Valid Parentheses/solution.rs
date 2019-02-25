impl Solution {
    pub fn is_valid(s: String) -> bool {
        use std::collections::VecDeque;
        let mut stack = VecDeque::new();
        for c in s.chars() {
            if c == '(' || c == '[' || c == '{' {
                stack.push_back(c);
            } else {
                match stack.pop_back() {
                    None => return false,
                    Some(ch) => {
                        if c == ')' && ch != '(' || c == ']' && ch != '[' || c == '}' && ch != '{' {
                            return false;
                        }
                    }
                }
            }
        }
        stack.len() == 0
    }
}
