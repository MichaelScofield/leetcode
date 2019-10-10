impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        // https://github.com/MichaelScofield/leetcode/blob/master/p0367.%20Valid%20Perfect%20Square/solution.rs
        fn is_perfect_square(num: i64) -> bool {
            match num {
                0 | 1 => true,
                _ => {
                    let mut i = 0;
                    let mut j = num;
                    let mut last_x = 0;
                    loop {
                        let x = (i + j) / 2;
                        if x == last_x {
                            break;
                        }
                        last_x = x;
                        let y = x * x;
                        if y == num {
                            return true;
                        } else if y < num {
                            i = x;
                        } else {
                            j = x;
                        }
                    }
                    false
                }
            }
        }

        let c = c as i64;
        for i in 0..c / 2 + 1 {
            if i * i <= c {
                if is_perfect_square(c - i * i) {
                    return true;
                }
            } else {
                break;
            }
        }
        false
    }
}
