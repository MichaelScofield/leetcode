impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let l = bits.len();
        let mut i = 0;
        while i < l {
            if bits[i] == 1 {
                i += 2;
            } else {
                if i == l - 1 {
                    return true;
                }
                i += 1;
            }
        }
        false
    }
}
