impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid[0].len();
        let n = grid.len();
        if m < 3 || n < 3 {
            return 0;
        }
        let mut num_magic_squares = 0;
        for i in 0..m - 2 {
            for j in 0..n - 2 {
                if Self::is_magic_square(&grid, i, j) {
                    num_magic_squares += 1;
                }
            }
        }
        num_magic_squares
    }

    fn is_magic_square(grid: &Vec<Vec<i32>>, x: usize, y: usize) -> bool {
        let mut vs = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut row_sum = 0;
        for j in y..y + 3 {
            row_sum = 0;
            for i in x..x + 3 {
                let v = grid[j][i];
                if v > 9 || v < 1 {
                    return false;
                }
                vs[v as usize] = 0;

                row_sum += v;
            }
            if row_sum != 15 {
                return false;
            }
        }
        for v in vs {
            if v != 0 {
                return false;
            }
        }

        let mut col_sum = 0;
        for i in x..x + 3 {
            col_sum = 0;
            for j in y..y + 3 {
                col_sum += grid[j][i];
            }
            if col_sum != 15 {
                return false;
            }
        }

        // check diagonal sum
        if grid[y][x] + grid[y + 1][x + 1] + grid[y + 2][x + 2] != 15 {
            return false;
        }
        if grid[y][x + 2] + grid[y + 1][x + 1] + grid[y + 2][x] != 15 {
            return false;
        }
        true
    }
}
