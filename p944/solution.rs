impl Solution {
    pub fn min_deletion_size(a: Vec<String>) -> i32 {
        let mut delete_columns = Vec::new();
        let columns = a[0].len();
        for column in 0..columns {
            let mut last_char_code = 0;
            for s in &a {
                let current_char_code = s.chars().nth(column).unwrap() as u32;
                if current_char_code < last_char_code {
                    delete_columns.push(column);
                    break;
                } else {
                    last_char_code = current_char_code;
                }
            }
        }
        return delete_columns.len() as i32;
    }
}
