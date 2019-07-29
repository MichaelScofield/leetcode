impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let nums = (row_index + 1) as usize;
        let mut target_row = vec![1; nums];
        for i in 0..nums {
            for j in (0..i).rev() {
                if j == 0 {
                    target_row[j] = 1;
                } else {
                    target_row[j] += target_row[j - 1];
                }
            }
        }
        target_row
    }
}
