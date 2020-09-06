impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        if m == 0 {
            return Vec::with_capacity(0);
        }
        let n = matrix[0].len();
        if n == 0 {
            return Vec::with_capacity(0);
        }
        let mut arr = Vec::with_capacity(m * n);
        let rings = std::cmp::min(m / 2 + m % 2, n / 2 + n % 2);
        for i in 0..rings {
            let left_bound = i;
            let right_bound = n - i;
            let upper_bound = i;
            let lower_bound = m - i;
            let mut x = i;
            let mut y = i;
            while y < right_bound {
                arr.push(matrix[x][y]);
                if y + 1 == right_bound {
                    break;
                }
                y += 1;
            }

            if x + 1 == lower_bound {
                continue;
            }
            x += 1;
            while x < lower_bound {
                arr.push(matrix[x][y]);
                if x + 1 == lower_bound {
                    break;
                }
                x += 1;
            }

            if y == left_bound {
                continue;
            }
            y -= 1;
            while y >= left_bound {
                arr.push(matrix[x][y]);
                if y == left_bound {
                    break;
                }
                y -= 1;
            }

            x -= 1;
            while x > upper_bound {
                arr.push(matrix[x][y]);
                x -= 1
            }
        }
        arr
    }
}
