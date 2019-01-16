impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let is_negative = num < 0;
        let num_abs: i32 = num.abs();
        for i in 0..32 {
            let j = (1 << i) - 1;
            if j - num_abs >= 0 {
                let complement = j - num_abs;
                return if is_negative { -complement } else { complement };
            }
        }
        panic!()
    }
}
