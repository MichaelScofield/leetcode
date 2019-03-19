impl Solution {
    pub fn find_relative_ranks(nums: Vec<i32>) -> Vec<String> {
        let nums_copy = &mut nums.clone();
        nums_copy.sort();
        let l = nums.len();
        let mut ranks = Vec::new();
        for num in nums.iter() {
            let rank = nums_copy.binary_search(num).unwrap();
            if rank == l - 1 {
                ranks.push("Gold Medal".to_string());
            } else if rank == l - 2 {
                ranks.push("Silver Medal".to_string());
            } else if rank == l - 3 {
                ranks.push("Bronze Medal".to_string());
            } else {
                ranks.push((l - rank).to_string());
            }
        }
        ranks
    }
}
