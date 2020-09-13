impl Solution {
    pub fn min_number(nums: Vec<i32>) -> String {
        if nums.len() == 0 {
            return "".to_string();
        }
        let nums = &mut { nums };
        nums.sort_unstable_by(|&a, &b| {
            let a = a.to_string();
            let b = b.to_string();
            let mut ab = a.clone();
            ab.push_str(&b);
            let mut ba = b.clone();
            ba.push_str(&a);
            ab.cmp(&ba)
        });
        let mut s = String::new();
        for num in nums {
            s.push_str(&num.to_string());
        }
        s
    }
}
