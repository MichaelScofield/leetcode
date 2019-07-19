impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let len = s.len();
        let mut reversed = String::with_capacity(len);
        let chars = s.chars().collect::<Vec<char>>();
        let mut c = 0;
        let mut is_last_part = false;
        while !is_last_part {
            for i in 0..k {
                let j = if c % 2 == 0 { (c + 1) * k - i - 1 } else { c * k + i } as usize;
                if j >= len {
                    is_last_part = true;
                } else {
                    reversed.push(chars[j]);
                }
            }
            c += 1;
        }
        reversed
    }
}
