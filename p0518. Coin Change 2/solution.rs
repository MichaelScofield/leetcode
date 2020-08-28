impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        if amount == 0 {
            return 1;
        }
        if coins.len() == 0 {
            return 0;
        }
        let n = coins.len();
        let amount = amount as usize;
        let mut dp = vec![vec![0; amount + 1]; n + 1];
        for i in 1..=n {
            dp[i][0] = 1;
        }
        for i in 1..=n {
            let coin = coins[i - 1] as usize;
            for j in 1..=amount {
                if j < coin {
                    dp[i][j] = dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j] + dp[i][j - coin];
                }
            }
        }
        dp[n][amount]
    }
}
