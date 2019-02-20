impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut point_with_distance = Vec::<((i32, i32), i32)>::new();
        for point in points {
            let x = point[0];
            let y = point[1];
            let distance = x * x + y * y;
            point_with_distance.push(((x, y), distance));
        }
        point_with_distance.sort_unstable_by_key(|((_, _), distance)| *distance);

        let k = k as usize;
        let mut k_closest_points = Vec::<Vec<i32>>::with_capacity(k);
        for (i, ((x, y), _)) in point_with_distance.iter().enumerate() {
            if i < k {
                k_closest_points.push(vec![*x, *y]);
            } else {
                break;
            }
        }
        k_closest_points
    }
}
