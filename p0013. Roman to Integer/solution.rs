impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        use std::collections::HashMap;
        let mut mappings = HashMap::with_capacity(7);
        mappings.insert('I', 1);
        mappings.insert('V', 5);
        mappings.insert('X', 10);
        mappings.insert('L', 50);
        mappings.insert('C', 100);
        mappings.insert('D', 500);
        mappings.insert('M', 1000);

        let mut sum = 0;
        let chars: Vec<char> = s.chars().collect();
        let mut i = chars.len();
        let mut add = true;
        while i > 0 {
            let j = i - 1;
            let x = mappings[&chars[j]];
            if add {
                sum += x;
            } else {
                sum -= x;
                add = true;
            }
            if j > 0 {
                if mappings[&chars[j - 1]] < x {
                    add = false;
                }
            }
            i -= 1;
        }
        sum
    }
}
