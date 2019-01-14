impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut char_indexes = Vec::new();
        for (i, ch) in s.chars().enumerate() {
            if ch == c {
                char_indexes.push(i);
            }
        }
        let mut shortest_distances = Vec::with_capacity(s.len());
        for i in 0..s.len() {
            let mut shortest_distance: i32 = i32::max_value();
            for j in &char_indexes {
                let dist = (*j as i32 - i as i32).abs();
                if dist < shortest_distance {
                    shortest_distance = dist;
                }
            }
            shortest_distances.push(shortest_distance);
        }
        shortest_distances
    }
}
