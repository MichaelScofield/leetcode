impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut perimeter = 0;
        for j in 0..n {
            let row = &grid[j];
            for i in 0..m {
                if row[i] == 1 {
                    if j == 0 || grid[j - 1][i] == 0 {
                        perimeter += 1;
                    }
                    if j == n - 1 || grid[j + 1][i] == 0 {
                        perimeter += 1;
                    }
                    if i == 0 || row[i - 1] == 0 {
                        perimeter += 1;
                    }
                    if i == m - 1 || row[i + 1] == 0 {
                        perimeter += 1;
                    }
                }
            }
        }
        perimeter
    }
}
