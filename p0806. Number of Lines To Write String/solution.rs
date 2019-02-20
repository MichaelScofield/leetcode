impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut solution = Vec::with_capacity(2);
        let mut lines = 1;
        let mut chars_sum = 0;
        for c in s.chars() {
            let to_write = widths[c as usize - 'a' as usize];
            if chars_sum + to_write > 100 {
                chars_sum = to_write;
                lines += 1;
            } else {
                chars_sum += to_write;
            }
        }
        solution.push(lines);
        solution.push(chars_sum);
        solution
    }
}
