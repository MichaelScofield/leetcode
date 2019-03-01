impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut pascal_triangle: Vec<Vec<i32>> = Vec::with_capacity(num_rows);
        for i in 0..num_rows {
            let mut current_row = Vec::with_capacity(i + 1);
            if i > 0 {
                let last_row = &pascal_triangle[i - 1];
                for j in 0..last_row.len() {
                    current_row.push(last_row[j] + if j > 0 { last_row[j - 1] } else { 0 });
                }
            }
            current_row.push(1);
            pascal_triangle.push(current_row);
        }
        pascal_triangle
    }
}
