impl Solution {
    pub fn is_ugly(num: i32) -> bool {
        if num <= 0 {
            return false;
        }
        let mut num = num;
        loop {
            if num % 2 == 0 {
                num /= 2;
            } else if num % 3 == 0 {
                num /= 3;
            } else if num % 5 == 0 {
                num /= 5;
            } else {
                return num == 1;
            }
        }
    }
}
