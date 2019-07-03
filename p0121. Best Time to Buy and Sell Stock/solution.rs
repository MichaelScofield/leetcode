impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 1 {
            return 0;
        }
        let mut min_price = prices[0];
        let mut max_profit = 0;
        for i in 1..prices.len() {
            let price = prices[i];
            if price < min_price {
                min_price = price;
            } else {
                let profit = price - min_price;
                if profit > max_profit {
                    max_profit = profit;
                }
            }
        }
        max_profit
    }
}
