impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut m = n;
        let mut last_bit = -1;
        while m > 0 {
            let bit = m % 2;
            if last_bit == bit {
                return false;
            } else {
                last_bit = bit;
            }
            m /= 2;
        }
        true
    }
}
