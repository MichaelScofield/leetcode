impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        match x {
            0 | 1 => x,
            _ => {
                let mut i = 0;
                let mut j = x;
                let x = x as i64;
                let mut n = -1;
                loop {
                    let m = (i + j) / 2;
                    if n == m {
                        break;
                    }
                    n = m;
                    let y: i64 = m as i64 * m as i64;
                    if y == x {
                        return m;
                    } else if y < x {
                        i = m;
                    } else {
                        j = m;
                    }
                }
                n
            }
        }
    }
}
