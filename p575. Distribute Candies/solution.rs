impl Solution {
    pub fn distribute_candies(candies: Vec<i32>) -> i32 {
        let sister_candies = (candies.len() / 2) as i32;
        let mut kind = 0;
        use std::collections::HashSet;
        let mut candies_kinds = HashSet::new();
        for candy in candies.into_iter() {
            if candies_kinds.insert(candy) {
                kind += 1;
                if kind > sister_candies {
                    return sister_candies;
                }
            }
        }
        kind
    }
}
