impl Solution {
    pub fn transpose(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = a.len();
        let cols = a[0].len();
        let mut a_t = Vec::with_capacity(cols);
        for j in 0..cols {
            let mut row = Vec::with_capacity(rows);
            for i in 0..rows {
                let e = a[i][j];
                row.push(e)
            }
            a_t.push(row);
        }
        a_t
    }
}
