impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut self_dividing_nums = Vec::new();
        for i in left..right + 1 {
            if i == 0 {
                continue;
            }
            let mut j = i;
            let mut is_self_dividing_num = true;
            loop {
                let remaining = j % 10;
                if remaining == 0 {
                    is_self_dividing_num = false;
                    break;
                }
                is_self_dividing_num &= i % remaining == 0;
                if !is_self_dividing_num {
                    break;
                }
                j = (j - remaining) / 10;
                if j == 0 {
                    break;
                }
            }
            if is_self_dividing_num {
                self_dividing_nums.push(i);
            }
        }
        return self_dividing_nums;
    }
}
