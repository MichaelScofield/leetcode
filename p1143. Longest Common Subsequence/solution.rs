impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        assert!(text1.len() > 0 && text2.len() > 0);
        let text1 = text1.as_bytes();
        let text2 = text2.as_bytes();
        let m = text1.len();
        let n = text2.len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = std::cmp::max(
                    dp[i - 1][j - 1] + if text1[i - 1] == text2[j - 1] { 1 } else { 0 },
                    std::cmp::max(dp[i][j - 1], dp[i - 1][j]));
            }
        }
        dp[m][n]
    }
}
