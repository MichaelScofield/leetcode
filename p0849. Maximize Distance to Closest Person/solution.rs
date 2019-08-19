impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let l = seats.len();
        let mut dists = vec![l; l];
        let mut last_left_person_index = 0;
        let mut last_right_person_index = l - 1;
        for i in 0..l {
            if seats[i] == 1 {
                last_left_person_index = i;
            } else {
                if seats[last_left_person_index] == 1 {
                    dists[i] = std::cmp::min(dists[i], i - last_left_person_index);
                }
            }

            let j = l - 1 - i;
            if seats[j] == 1 {
                last_right_person_index = j;
            } else {
                if seats[last_right_person_index] == 1 {
                    dists[j] = std::cmp::min(dists[j], last_right_person_index - j);
                }
            }
        }
        *dists.iter().filter(|&x| *x != l).max().unwrap() as i32
    }
}
