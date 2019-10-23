impl Solution {
    pub fn buddy_strings(a: String, b: String) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let a = a.as_bytes();
        let b = b.as_bytes();
        let mut p = None;
        let mut q = None;
        for i in 0..a.len() {
            if a[i] != b[i] {
                if p == None {
                    p = Some(i);
                } else {
                    if q == None {
                        q = Some(i);
                    } else {
                        return false;
                    }
                }
            }
        }
        if p == None && q == None {
            use std::collections::HashSet;
            let mut set = HashSet::with_capacity(a.len());
            for x in a {
                if !set.insert(x) {
                    return true;
                }
            }
            return false;
        }
        if p.is_some() && q.is_some() {
            let p = p.unwrap();
            let q = q.unwrap();
            return a[p] == b[q] && a[q] == b[p];
        }
        false
    }
}
