impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut y: i32 = 0;
        while x != 0 {
            if let Some(z) = y.checked_mul(10).and_then(|a| a.checked_add(x % 10)) {
                y = z;
            } else {
                return 0;
            }
            x /= 10;
        }
        y
    }
}
