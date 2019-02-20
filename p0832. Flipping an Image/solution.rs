impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut b: Vec<Vec<i32>> = Vec::new();
        for row in a {
            let mut new_row: Vec<i32> = Vec::new();
            for pixel in row {
                new_row.push(if pixel > 0 { 0 } else { 1 });
            }
            new_row.reverse();
            b.push(new_row);
        }
        return b;
    }
}
