impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let m = matrix[0].len();
        let n = matrix.len();
        for i in 0..n {
            let row = &matrix[i];
            for j in 0..m {
                if i == 0 || j < i {
                    let expected = row[j];
                    let mut x = j + 1;
                    let mut y = i + 1;
                    while x < m && y < n {
                        if matrix[y][x] != expected {
                            return false;
                        } else {
                            x += 1;
                            y += 1;
                        }
                    }
                }
            }
        }
        true
    }
}
