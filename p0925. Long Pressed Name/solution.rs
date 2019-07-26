impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let l1 = name.len();
        let l2 = typed.len();
        if l1 == 0 && l2 == 0 {
            return true;
        }
        if l1 > l2 || l1 == 0 && l2 > 0 || l1 > 0 && l2 == 0 {
            return false;
        }
        let mut i = 0;
        let mut j = 0;
        let name: Vec<char> = name.chars().collect();
        let typed: Vec<char> = typed.chars().collect();
        let mut last_char = ' ';
        while j < l2 {
            let expect = if i < l1 { name[i] } else { name[l1 - 1] };
            if typed[j] == expect {
                i += 1;
                j += 1;
                last_char = expect;
            } else {
                if typed[j] == last_char {
                    j += 1;
                } else {
                    return false;
                }
            }
        }
        i >= l1
    }
}
