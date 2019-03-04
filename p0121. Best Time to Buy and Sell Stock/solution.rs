impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let l = prices.len();
        for (i, price) in prices.iter().enumerate() {
            for j in i + 1..l {
                let profit = prices[j] - *price;
                if profit > max_profit {
                    max_profit = profit;
                }
            }
        }
        max_profit
    }
}
