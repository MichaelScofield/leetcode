impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        let n = n as i64;
        for i in 1..33 {
            let j = (1i64 << i) - 1;
            if j >= n {
                return (j - n) as i32;
            }
        }
        panic!()
    }
}
