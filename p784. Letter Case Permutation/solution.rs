impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let chars = s.chars().collect();
        let mut result = Vec::new();
        Self::permutation(&mut Vec::new(), &chars, 0, &mut result);
        result
    }

    fn permutation(prefix: &mut Vec<char>, chars: &Vec<char>, i: usize, result: &mut Vec<String>) {
        let l = chars.len();
        if i == l {
            result.push(prefix.iter().collect());
            return;
        }
        let mut i = i;
        while i < l {
            let c = chars[i];
            i += 1;
            if !c.is_ascii_alphabetic() {
                prefix.push(c);
            } else {
                let mut prefix1 = prefix.clone();
                prefix1.push(c);
                Self::permutation(&mut prefix1, chars, i, result);

                let mut prefix2 = prefix.clone();
                if c.is_uppercase() {
                    prefix2.push(c.to_ascii_lowercase());
                } else {
                    prefix2.push(c.to_ascii_uppercase());
                }
                Self::permutation(&mut prefix2, chars, i, result);
                return;
            }
        }
        result.push(prefix.iter().collect());
    }
}
