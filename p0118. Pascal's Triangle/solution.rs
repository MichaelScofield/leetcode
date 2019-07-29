impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut pascal_triangle = Vec::with_capacity(num_rows);
        for i in 0..num_rows {
            if i == 0 {
                pascal_triangle.push(vec![1]);
            } else {
                let last_row = &pascal_triangle[i - 1];
                let mut new_row = Vec::with_capacity(last_row.len() + 1);
                for j in 0..last_row.len() {
                    if j == 0 {
                        new_row.push(1);
                    } else {
                        new_row.push(last_row[j - 1] + last_row[j]);
                    }
                }
                new_row.push(1);
                pascal_triangle.push(new_row);
            }
        }
        pascal_triangle
    }
}
