impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        let mut chars = Vec::new();
        let mut x = num.abs();
        while x != 0 {
            let r = x % 7;
            chars.push(std::char::from_digit(r as u32, 10).unwrap());
            x /= 7;
        }
        if num < 0 {
            chars.push('-');
        }
        chars.reverse();
        chars.iter().collect()
    }
}
