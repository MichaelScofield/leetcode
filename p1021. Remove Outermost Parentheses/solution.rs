impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut stack = Vec::new();
        let mut start = None;
        let mut result = String::new();
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                stack.push(c);
                if start == None {
                    start = Some(i);
                }
            } else if c == ')' {
                stack.pop();
                if stack.is_empty() {
                    result.push_str(&s[start.unwrap() + 1..i]);
                    start = None;
                }
            }
        }
        result
    }
}
