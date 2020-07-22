impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n < 1 {
            return Vec::with_capacity(0);
        }
        use std::collections::HashSet;
        let mut parenthesis_set = HashSet::new();
        parenthesis_set.insert(vec![b'(', b')']);
        for _i in 2..=n {
            let mut new_set = HashSet::new();
            for parenthesis in parenthesis_set {
                let l = parenthesis.len();
                let mut r1 = vec![b'0'; l + 2];
                r1[1..=l].copy_from_slice(&parenthesis);
                r1[0] = b'(';
                r1[l + 1] = b')';
                new_set.insert(r1);

                let mut r2 = vec![b'0'; l + 2];
                r2[..l].copy_from_slice(&parenthesis);
                r2[l] = b'(';
                r2[l + 1] = b')';
                new_set.insert(r2);

                for (i, &x) in parenthesis.iter().enumerate() {
                    if x == b'(' {
                        let mut r = parenthesis.clone();
                        r.insert(i + 1, b'(');
                        r.insert(i + 2, b')');
                        new_set.insert(r);
                    }
                }
            }
            parenthesis_set = new_set;
        }
        parenthesis_set.into_iter()
            .map(|parenthesis| String::from_utf8(parenthesis).unwrap())
            .collect()
    }
}
