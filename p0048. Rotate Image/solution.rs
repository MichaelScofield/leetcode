impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        fn rotate(matrix: &mut Vec<Vec<i32>>,
                  upper_bound: usize, lower_bound: usize, left_bound: usize, right_bound: usize) {
            if left_bound >= right_bound {
                return;
            }
            let width = right_bound - left_bound;
            for i in left_bound..right_bound {
                let x1 = std::cmp::min(i + width, right_bound);
                let y1 = upper_bound + (width - (x1 - i));
                let t1 = matrix[y1][x1];
                matrix[y1][x1] = matrix[upper_bound][i];

                let y2 = std::cmp::min(y1 + width, lower_bound);
                let x2 = right_bound - (width - (y2 - y1));
                let t2 = matrix[y2][x2];
                matrix[y2][x2] = t1;

                let x3 = std::cmp::max(x2 as i32 - width as i32, left_bound as i32) as usize;
                let y3 = lower_bound - (width - (x2 - x3));
                let t3 = matrix[y3][x3];
                matrix[y3][x3] = t2;

                matrix[upper_bound][i] = t3;
            }
            rotate(matrix, upper_bound + 1, lower_bound - 1,
                   left_bound + 1, right_bound - 1);
        }
        rotate(matrix, 0, matrix.len() - 1,
               0, matrix.len() - 1);
    }
}
