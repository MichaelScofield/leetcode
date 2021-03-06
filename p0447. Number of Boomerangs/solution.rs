impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        fn calc_distance(x: &Vec<i32>, y: &Vec<i32>) -> i32 {
            (x[0] - y[0]).abs().pow(2) + (x[1] - y[1]).abs().pow(2)
        }
        let mut num = 0;
        let l = points.len();
        for i in 0..l {
            for j in 0..l {
                if j != i {
                    for k in 0..l {
                        if k != i && k != j {
                            let distance_i_j = calc_distance(&points[i], &points[j]);
                            let distance_i_k = calc_distance(&points[i], &points[k]);
                            if distance_i_j == distance_i_k {
                                num += 1;
                            }
                        }
                    }
                }
            }
        }
        num
    }
}
