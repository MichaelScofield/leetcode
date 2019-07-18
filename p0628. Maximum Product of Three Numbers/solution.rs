impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let nums = &mut { nums };
        nums.sort();
        let len = nums.len();
        match len {
            0 | 1 | 2 | 3 => nums.iter().product(),
            _ => {
                let product1 = nums[0] * nums[1] * nums[2];
                let product2 = nums[0] * nums[1] * nums[len - 1];
                let product3 = nums[0] * nums[len - 2] * nums[len - 1];
                let product4 = nums[len - 3] * nums[len - 2] * nums[len - 1];
                std::cmp::max(std::cmp::max(product1, product2),
                              std::cmp::max(product3, product4))
            }
        }
    }
}
