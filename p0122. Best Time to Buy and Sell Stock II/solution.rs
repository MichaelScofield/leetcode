impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 1 {
            return 0;
        }
        let mut min_price = (0, prices[0]);
        let mut max_price = (0, prices[0]);
        let mut max_profit = 0;
        for i in 1..prices.len() {
            let price = prices[i];
            if price > prices[i - 1] {
                max_profit += price - prices[i - 1];
            }
            if price < min_price.1 {
                min_price = (i, price);
            }
            if price > max_price.1 {
                max_price = (i, price);
            }
        }
        if max_price.0 > min_price.0 {
            std::cmp::max(max_profit, max_price.1 - min_price.1)
        } else {
            max_profit
        }
    }
}
