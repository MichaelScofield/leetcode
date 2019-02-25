impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let nums = &mut { nums };
        nums.sort();

        let mut disappeared_nums = Vec::new();
        let mut expect = 1;
        let mut i = 0;
        while i < nums.len() {
            if nums[i] < expect {
                i += 1;
            } else {
                if nums[i] > expect {
                    disappeared_nums.push(expect);
                } else {
                    i += 1;
                }
                expect += 1;
            }
        }
        let l = nums.len() as i32;
        if expect <= l {
            for i in expect..l + 1 {
                disappeared_nums.push(i);
            }
        }
        disappeared_nums
    }
}
