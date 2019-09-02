impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        ((((1 + 8 * n as i64) as f64).sqrt() - 1f64) / 2f64) as i32
    }
}
