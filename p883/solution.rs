impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();

        let mut xy = 0;
        let mut xz = 0;
        for i in 0..n {
            let row = &grid[i];
            let mut max_z_in_y = 0;
            for j in 0..n {
                let cell = row[j];
                if cell > 0 {
                    xy += 1;
                    if cell > max_z_in_y {
                        max_z_in_y = cell;
                    }
                }
            }
            xz += max_z_in_y;
        }

        let mut yz = 0;
        for j in 0..n {
            let mut max_z_in_x = 0;
            for i in 0..n {
                let cell = grid[i][j];
                if cell > max_z_in_x {
                    max_z_in_x = cell;
                }
            }
            yz += max_z_in_x;
        }

        xy + yz + xz
    }
}
