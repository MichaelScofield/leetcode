impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        use std::collections::HashMap;
        let mut occurrences = HashMap::new();
        for x in arr {
            let occurrence = occurrences.entry(x).or_insert(1);
            *occurrence += 1;
        }
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for (_, occurrence) in occurrences {
            if !set.insert(occurrence) {
                return false;
            }
        }
        true
    }
}
