struct KthLargest {
    k: usize,
    nums: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        KthLargest { k: k as usize, nums }
    }

    fn add(&mut self, val: i32) -> i32 {
        let nums = &mut self.nums;
        nums.push(val);
        nums.sort_unstable_by(|a, b| b.cmp(a));
        nums[self.k - 1]
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
