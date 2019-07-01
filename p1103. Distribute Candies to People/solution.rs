impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let mut ans = vec![0; num_people as usize];
        let mut candies = candies;
        let mut i = 0;
        while candies > 0 {
            let j = i % num_people;
            i += 1;
            ans[j as usize] += std::cmp::min(candies, i);
            candies -= i;
        }
        ans
    }
}
