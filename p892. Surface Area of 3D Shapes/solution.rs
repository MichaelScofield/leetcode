impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut surfaces = 0;
        for (i, row) in grid.iter().enumerate() {
            for (j, h) in row.iter().enumerate() {
                let h = *h;
                // bottom and top
                if h > 0 {
                    surfaces += 2;
                }

                // up and down
                if i == 0 {
                    surfaces += h;
                } else if grid[i - 1][j] < h {
                    surfaces += h - grid[i - 1][j];
                }
                if i == rows - 1 {
                    surfaces += h;
                } else if grid[i + 1][j] < h {
                    surfaces += h - grid[i + 1][j];
                }

                // left and right
                if j == 0 {
                    surfaces += h;
                } else if grid[i][j - 1] < h {
                    surfaces += h - grid[i][j - 1];
                }
                if j == cols - 1 {
                    surfaces += h;
                } else if grid[i][j + 1] < h {
                    surfaces += h - grid[i][j + 1];
                }
            }
        }
        surfaces
    }
}
