impl Solution {
    pub fn is_happy(n: i32) -> bool {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let mut n = n;
        while n != 1 {
            if !set.insert(n) {
                return false;
            }
            let mut ss = 0;
            while n != 0 {
                let r = n % 10;
                ss += r * r;
                n /= 10;
            }
            n = ss;
        }
        true
    }
}
