impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let chars_a = a.chars().collect::<Vec<char>>();
        let chars_b = b.chars().collect::<Vec<char>>();
        let mut sum = Vec::with_capacity(
            std::cmp::max(chars_a.len(), chars_b.len()) + 1);
        let mut i = chars_a.len() as i32 - 1;
        let mut j = chars_b.len() as i32 - 1;
        let mut carry = 0;
        while i >= 0 || j >= 0 {
            let char_a = if i >= 0 { chars_a[i as usize].to_digit(10).unwrap() } else { 0 };
            let char_b = if j >= 0 { chars_b[j as usize].to_digit(10).unwrap() } else { 0 };
            let add = char_a + char_b + carry;
            if add == 3 {
                sum.push('1');
                carry = 1;
            } else if add == 2 {
                sum.push('0');
                carry = 1;
            } else {
                sum.push(std::char::from_digit(add, 10).unwrap());
                carry = 0;
            }
            i -= 1;
            j -= 1;
        }
        if carry == 1 {
            sum.push('1');
        }
        sum.reverse();
        sum.iter().collect::<String>()
    }
}
