impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }
        let mut a = 1;
        let mut b = 2;
        for _i in 3..n {
            let c = a + b;
            a = b;
            b = c;
        }
        a + b
    }
}
