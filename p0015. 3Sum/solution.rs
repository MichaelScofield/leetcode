impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for &num in nums.iter() {
            let count = map.entry(num).or_insert(0);
            *count += 1;
        }

        use std::collections::HashSet;
        let mut solutions = HashSet::new();
        for (&num1, _) in map.iter() {
            let expect = -num1;
            for (&num2, &count) in map.iter() {
                if num2 != num1 || count > 1 {
                    let residue = expect - num2;
                    let value = map.get(&residue);
                    if let Some(&count) = value {
                        if residue == num2 && residue == num1 && count > 2 ||
                            num2 != num1 && (residue == num2 || residue == num1) && count > 1 ||
                            residue != num2 && residue != num1 {
                            let mut solution = vec![num1, num2, residue];
                            solution.sort();
                            solutions.insert(solution);
                        }
                    }
                }
            }
        }
        use std::iter::FromIterator;
        Vec::from_iter(solutions.into_iter())
    }
}
