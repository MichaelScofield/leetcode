impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        assert_eq!(pushed.len(), popped.len());
        let n = pushed.len();
        if n == 0 {
            return true;
        }
        let mut stack = vec![];
        let mut x = 0;
        for pop in popped {
            if let Some(&v) = stack.last() {
                if v == pop {
                    stack.pop();
                    continue;
                }
            }
            let mut is_found = false;
            while x < n {
                if pop != pushed[x] {
                    stack.push(pushed[x]);
                    x += 1;
                } else {
                    is_found = true;
                    x += 1;
                    break;
                }
            }
            if !is_found {
                return false;
            }
        }
        stack.is_empty()
    }
}
