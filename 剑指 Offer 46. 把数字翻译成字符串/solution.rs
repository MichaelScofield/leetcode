impl Solution {
    pub fn translate_num(num: i32) -> i32 {
        assert!(num >= 0);
        let num = num.to_string();
        let chars: Vec<u32> = num.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let len = chars.len();
        let mut dp = vec![0; len + 1];
        dp[1] = 1;
        for i in 2..=len {
            dp[i] = dp[i - 1];
            let x = chars[i - 2] * 10 + chars[i - 1];
            if x <= 25 && x >= 10 {
                dp[i] += std::cmp::max(dp[i - 2], 1);
            }
        }
        dp[len]
    }
}
