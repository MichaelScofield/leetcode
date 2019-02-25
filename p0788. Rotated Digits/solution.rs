impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        let mut good_nums = 0;
        'l: for i in 1..n + 1 {
            let mut x = 0;
            let mut j = i;
            let mut d = 0;
            loop {
                let r = match j % 10 {
                    0 | 1 | 8 => j % 10,
                    2 => 5,
                    5 => 2,
                    6 => 9,
                    9 => 6,
                    _ => continue 'l
                };
                x += r * 10_i32.pow(d);
                j /= 10;
                if j == 0 {
                    break;
                }
                d += 1;
            }
            if x != i {
                good_nums += 1;
            }
        }
        good_nums
    }
}
