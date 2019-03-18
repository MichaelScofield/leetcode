impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if trust.len() == 0 {
            return if n == 1 { 1 } else { -1 };
        }
        use std::collections::HashMap;
        let mut trust_mappings = HashMap::new();
        let mut trust_counts = HashMap::new();
        for x in trust {
            let persons = trust_mappings.entry(x[0]).or_insert(Vec::<i32>::new());
            persons.push(x[1]);

            let count = trust_counts.entry(x[1]).or_insert(0);
            *count += 1;
        }
        for (person, count) in trust_counts {
            if count == n - 1 && !trust_mappings.contains_key(&person) {
                return person;
            }
        }
        -1
    }
}
