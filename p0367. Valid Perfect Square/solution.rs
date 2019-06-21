impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        match num {
            0 | 1 => true,
            _ => {
                let mut i = 0;
                let mut j = num;
                let mut last_x = 0;
                let num = num as i64;
                loop {
                    let x = (i + j) / 2;
                    if x == last_x {
                        break;
                    }
                    last_x = x;
                    let y: i64 = x as i64 * x as i64;
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
}
