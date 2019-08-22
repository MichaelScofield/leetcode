impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let l1 = s.len();
        if l1 == 0 {
            return true;
        }
        let l2 = t.len();
        let mut i = 0;
        let mut j = 0;
        let sc = s.chars().collect::<Vec<char>>();
        let tc = t.chars().collect::<Vec<char>>();
        while i < l1 && j < l2 {
            if sc[i] == tc[j] {
                i += 1;
            }
            j += 1;
            if i == l1 {
                return true;
            }
        }
        false
    }
}
