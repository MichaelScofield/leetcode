impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let digits1: Vec<u32> = num1.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let digits2: Vec<u32> = num2.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut i = digits1.len() as i32 - 1;
        let mut j = digits2.len() as i32 - 1;
        let mut sum_digits = Vec::with_capacity(std::cmp::max(i, j) as usize + 1);
        let mut carry = 0;
        while i >= 0 || j >= 0 {
            let digit1 = if i >= 0 { digits1[i as usize] } else { 0 };
            let digit2 = if j >= 0 { digits2[j as usize] } else { 0 };
            let add = digit1 + digit2 + carry;
            carry = if add > 9 { 1 } else { 0 };
            sum_digits.push(add % 10);
            i -= 1;
            j -= 1;
        }
        if carry == 1 {
            sum_digits.push(1);
        }
        let mut sum = String::with_capacity(sum_digits.len());
        for digit in sum_digits.iter().rev() {
            sum.push(std::char::from_digit(*digit, 10).unwrap());
        }
        sum
    }
}
