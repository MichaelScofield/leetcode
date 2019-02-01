impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut bits = Vec::<u8>::new();
        let mut n = n;
        loop {
            if n == 0 {
                break;
            } else {
                bits.push((n % 2) as u8);
                n /= 2;
            }
        }
        let mut gap = 0;
        let mut p = None;
        for (i, bit) in bits.into_iter().enumerate() {
            if bit == 1 {
                if let Some(q) = p {
                    if i - q > gap {
                        gap = i - q;
                    }
                }
                p = Some(i);
            }
        }
        gap as i32
    }
}
