impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if candidates.len() < 1 {
            return Vec::with_capacity(0);
        }
        use std::collections::HashSet;
        fn combination_sum(candidates: &Vec<i32>, target: i32,
                           result: Vec<i32>, results: &mut HashSet<Vec<i32>>) {
            for &candidate in candidates.iter() {
                if target < candidate {
                    continue;
                }
                let mut result = result.clone();
                result.push(candidate);
                if target == candidate {
                    result.sort();
                    results.insert(result);
                } else {
                    combination_sum(&candidates, target - candidate, result, results);
                }
            }
        }
        let mut results = HashSet::new();
        let result = vec![];
        combination_sum(&candidates, target, result, &mut results);
        results.into_iter().collect()
    }
}
