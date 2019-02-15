impl Solution {
    pub fn is_monotonic(a: Vec<i32>) -> bool {
        let l = a.len();
        if l <= 1 {
            return true;
        }
        let trend_incr = 1;
        let trend_decr = 2;
        let mut expected_trend = 0;
        for i in 0..l - 1 {
            if a[i] == a[i + 1] {
                continue;
            }
            let trend = if a[i] < a[i + 1] { trend_incr } else { trend_decr };
            if expected_trend != 0 && expected_trend != trend {
                return false;
            } else {
                expected_trend = trend;
            }
        }
        true
    }
}
