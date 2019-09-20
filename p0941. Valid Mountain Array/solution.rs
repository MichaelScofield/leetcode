impl Solution {
    pub fn valid_mountain_array(a: Vec<i32>) -> bool {
        if a.len() < 3 {
            return false;
        }
        let mut has_up = false;
        let mut has_down = false;
        for i in 0..a.len() - 1 {
            if a[i] == a[i + 1] {
                return false;
            } else if a[i] < a[i + 1] {
                if has_down {
                    return false;
                }
                if !has_up {
                    has_up = true;
                }
            } else {
                if !has_up {
                    return false;
                }
                if !has_down {
                    has_down = true;
                }
            }
        }
        has_up && has_down
    }
}
