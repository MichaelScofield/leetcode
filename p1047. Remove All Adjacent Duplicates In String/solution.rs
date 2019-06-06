impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        use std::collections::VecDeque;
        let mut stack = VecDeque::new();
        for c in s.chars() {
            if let Some(&ch) = stack.back() {
                if ch == c {
                    stack.pop_back();
                    continue;
                }
            }
            stack.push_back(c);
        }
        stack.iter().collect()
    }
}
