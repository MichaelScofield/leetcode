impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut five_s = 0;
        let n = n as i64;
        for i in 1..n + 1 {
            let r = i % 10;
            if r == 0 || r == 5 {
                let mut j = i;
                loop {
                    if j % 5 == 0 {
                        five_s += 1;
                        j /= 5;
                    } else {
                        break;
                    }
                }
            }
        }
        five_s
    }
}
