impl Solution {
    pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
        let mut peak = 0;
        let mut peak_index = 0;
        for (i, x) in a.iter().enumerate() {
            if *x >= peak {
                peak = *x;
                peak_index = i as i32;
            } else {
                return peak_index;
            }
        }
        return peak_index;
    }
}
