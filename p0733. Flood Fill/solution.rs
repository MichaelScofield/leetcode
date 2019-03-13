impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let image = &mut { image };
        let n = image.len();
        let m = image[0].len();
        let sr = sr as usize;
        let sc = sc as usize;
        let expected_color = image[sr][sc];
        use std::collections::HashSet;
        let mut filled_pixels = HashSet::new();
        let mut seed_pixels = Vec::new();

        image[sr][sc] = new_color;
        filled_pixels.insert((sr, sc));
        seed_pixels.push((sr, sc));
        loop {
            if seed_pixels.is_empty() {
                break;
            }
            let mut new_seeds = Vec::new();
            { // make the closure inside a block, to release the borrow to "new_seeds"
                let mut try_fill = |(r, c): (usize, usize)| {
                    if image[r][c] == expected_color {
                        image[r][c] = new_color;
                        if filled_pixels.insert((r, c)) {
                            new_seeds.push((r, c));
                        }
                    }
                };

                for (r, c) in seed_pixels {
                    if r > 0 {
                        try_fill((r - 1, c));
                    }
                    if r < n - 1 {
                        try_fill((r + 1, c));
                    }
                    if c > 0 {
                        try_fill((r, c - 1));
                    }
                    if c < m - 1 {
                        try_fill((r, c + 1));
                    }
                }
            }
            seed_pixels = new_seeds;
        }
        image.to_owned()
    }
}
