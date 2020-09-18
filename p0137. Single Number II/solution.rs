impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        assert!(nums.len() > 0);
        let mut bits_count = vec![0; 32];
        for num in nums {
            for i in 0..32 {
                if num & (1 << i) != 0 {
                    bits_count[i] += 1;
                }
            }
        }

        let mut n = 0;
        for i in 0..32 {
            let p = bits_count[i] % 3;
            if p == 1 {
                n += 2i32.pow(i as u32);
            }
        }
        n
    }
}
